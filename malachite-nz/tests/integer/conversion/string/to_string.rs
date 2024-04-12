// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::repeat_n;
use malachite_base::num::arithmetic::traits::SaturatingSubAssign;
use malachite_base::num::basic::traits::{NegativeOne, One, Zero};
use malachite_base::num::conversion::string::to_string::BaseFmtWrapper as BaseBaseFmtWrapper;
use malachite_base::num::conversion::traits::{FromStringBase, ToStringBase};
use malachite_base::strings::{
    string_is_subset, ToBinaryString, ToDebugString, ToLowerHexString, ToOctalString,
    ToUpperHexString,
};
use malachite_base::test_util::generators::{
    signed_gen, signed_gen_var_2, signed_unsigned_pair_gen_var_5, signed_unsigned_pair_gen_var_7,
    signed_unsigned_unsigned_triple_gen_var_3, unsigned_gen_var_8, unsigned_pair_gen_var_9,
};
use malachite_nz::integer::Integer;
use malachite_nz::natural::conversion::string::to_string::BaseFmtWrapper;
use malachite_nz::platform::SignedLimb;
use malachite_nz::test_util::generators::{
    integer_gen, integer_unsigned_pair_gen_var_1, integer_unsigned_pair_gen_var_2,
    integer_unsigned_unsigned_triple_gen_var_1,
};
use num::BigInt;
use std::cmp::max;
use std::panic::catch_unwind;
use std::str::FromStr;

fn test_padding(mut s: &str, mut s_padded: &str, mut width: usize) {
    assert!(s_padded.len() >= width);
    assert_eq!(s.len() >= width, s == s_padded);
    let negative = s.starts_with('-');
    assert_eq!(s_padded.starts_with('-'), negative);
    if negative {
        s = &s[1..];
        s_padded = &s_padded[1..];
        width.saturating_sub_assign(1);
    }
    assert!(s_padded.ends_with(&s));
    assert!(s_padded.len() >= width);
    assert_eq!(s.len() >= width, s == s_padded);
    if s.len() < width {
        let diff = s_padded.len() - s.len();
        assert!(s_padded[..diff].chars().all(|c| c == '0'));
        assert_eq!(&s_padded[diff..], s);
    }
}

#[test]
pub fn test_to_string() {
    fn test(u: &str) {
        let x = Integer::from_str(u).unwrap();
        assert_eq!(x.to_string(), u);
        assert_eq!(x.to_debug_string(), u);
        assert_eq!(format!("{x:00}"), u);
        assert_eq!(format!("{x:00?}"), u);
    }
    test("0");
    test("2");
    test("123");
    test("1000");
    test("1000000");
    test("1000000000000000");
    test("-2");
    test("-123");
    test("-1000");
    test("-1000000");
    test("-1000000000000000");

    fn test_width(u: &str, width: usize, out: &str) {
        let x = Integer::from_str(u).unwrap();
        let s = x.to_string();
        assert_eq!(format!("{x:0width$}"), out);
        assert_eq!(format!("{x:0width$?}"), out);
        test_padding(&s, out, width);
    }
    test_width("0", 0, "0");
    test_width("0", 1, "0");
    test_width("0", 2, "00");
    test_width("0", 5, "00000");
    test_width("1000000", 0, "1000000");
    test_width("1000000", 1, "1000000");
    test_width("1000000", 2, "1000000");
    test_width("1000000", 3, "1000000");
    test_width("1000000", 4, "1000000");
    test_width("1000000", 5, "1000000");
    test_width("1000000", 6, "1000000");
    test_width("1000000", 7, "1000000");
    test_width("1000000", 8, "01000000");
    test_width("1000000", 10, "0001000000");
    test_width("1000000000000000", 0, "1000000000000000");
    test_width("1000000000000000", 1, "1000000000000000");
    test_width("1000000000000000", 16, "1000000000000000");
    test_width("1000000000000000", 20, "00001000000000000000");
    test_width("-1000000", 0, "-1000000");
    test_width("-1000000", 1, "-1000000");
    test_width("-1000000", 2, "-1000000");
    test_width("-1000000", 3, "-1000000");
    test_width("-1000000", 4, "-1000000");
    test_width("-1000000", 5, "-1000000");
    test_width("-1000000", 6, "-1000000");
    test_width("-1000000", 7, "-1000000");
    test_width("-1000000", 8, "-1000000");
    test_width("-1000000", 9, "-01000000");
    test_width("-1000000", 10, "-001000000");
    test_width("-1000000000000000", 0, "-1000000000000000");
    test_width("-1000000000000000", 1, "-1000000000000000");
    test_width("-1000000000000000", 16, "-1000000000000000");
    test_width("-1000000000000000", 20, "-0001000000000000000");
}

#[test]
fn to_string_properties() {
    integer_gen().test_properties(|x| {
        let s = x.to_string();
        assert_eq!(x.to_debug_string(), s);
        assert_eq!(x.to_string_base(10), s);
        assert_eq!(format!("{}", BaseFmtWrapper::new(&x, 10)), s);
        assert_eq!(format!("{:?}", BaseFmtWrapper::new(&x, 10)), s);
        assert_eq!(format!("{:00}", BaseFmtWrapper::new(&x, 10)), s);
        assert_eq!(format!("{:00?}", BaseFmtWrapper::new(&x, 10)), s);
        assert_eq!(BigInt::from(&x).to_string(), s);
        assert_eq!(rug::Integer::from(&x).to_string(), s);
        assert!(string_is_subset(&s, "-0123456789"));
        if x != 0 {
            assert!(!s.starts_with('0'));
        }
    });

    integer_unsigned_pair_gen_var_2().test_properties(|(x, width)| {
        let s = x.to_string();
        let s_padded = format!("{x:0width$}");
        test_padding(&s, &s_padded, width);
        assert_eq!(format!("{x:0width$?}"), s_padded);
        assert_eq!(
            format!("{:0width$}", BaseFmtWrapper::new(&x, 10), width = width),
            s_padded
        );
        assert_eq!(
            format!("{:0width$?}", BaseFmtWrapper::new(&x, 10), width = width),
            s_padded
        );
        assert_eq!(
            format!("{:0width$}", BigInt::from(&x), width = width),
            s_padded
        );
        assert_eq!(
            format!("{:0width$}", rug::Integer::from(&x), width = width),
            s_padded
        );
    });

    signed_gen::<SignedLimb>().test_properties(|x| {
        assert_eq!(Integer::from(x).to_string(), x.to_string());
    });

    signed_unsigned_pair_gen_var_5::<SignedLimb, usize>().test_properties(|(x, width)| {
        assert_eq!(
            format!("{:0width$}", Integer::from(x), width = width),
            format!("{x:0width$}")
        );
    });
}

#[test]
pub fn test_to_binary_string() {
    fn test(u: &str, out: &str, out_prefixed: &str) {
        let x = Integer::from_str(u).unwrap();
        assert_eq!(x.to_binary_string(), out);
        assert_eq!(format!("{x:00b}"), out);
        assert_eq!(format!("{x:#b}"), out_prefixed);
    }
    test("0", "0", "0b0");
    test("2", "10", "0b10");
    test("123", "1111011", "0b1111011");
    test("1000", "1111101000", "0b1111101000");
    test("1000000", "11110100001001000000", "0b11110100001001000000");
    test(
        "1000000000000000",
        "11100011010111111010100100110001101000000000000000",
        "0b11100011010111111010100100110001101000000000000000",
    );
    test("-2", "-10", "-0b10");
    test("-123", "-1111011", "-0b1111011");
    test("-1000", "-1111101000", "-0b1111101000");
    test(
        "-1000000",
        "-11110100001001000000",
        "-0b11110100001001000000",
    );
    test(
        "-1000000000000000",
        "-11100011010111111010100100110001101000000000000000",
        "-0b11100011010111111010100100110001101000000000000000",
    );

    fn test_width(u: &str, width: usize, out: &str, out_prefixed: &str) {
        let x = Integer::from_str(u).unwrap();
        let s = x.to_binary_string();
        assert_eq!(format!("{x:0width$b}"), out);
        assert_eq!(format!("{x:#0width$b}"), out_prefixed);
        test_padding(&s, out, width);
    }
    test_width("0", 0, "0", "0b0");
    test_width("0", 1, "0", "0b0");
    test_width("0", 2, "00", "0b0");
    test_width("0", 5, "00000", "0b000");
    test_width("1000", 0, "1111101000", "0b1111101000");
    test_width("1000", 1, "1111101000", "0b1111101000");
    test_width("1000", 10, "1111101000", "0b1111101000");
    test_width("1000", 12, "001111101000", "0b1111101000");
    test_width("1000", 14, "00001111101000", "0b001111101000");
    test_width(
        "1000000000000000",
        0,
        "11100011010111111010100100110001101000000000000000",
        "0b11100011010111111010100100110001101000000000000000",
    );
    test_width(
        "1000000000000000",
        1,
        "11100011010111111010100100110001101000000000000000",
        "0b11100011010111111010100100110001101000000000000000",
    );
    test_width(
        "1000000000000000",
        52,
        "0011100011010111111010100100110001101000000000000000",
        "0b11100011010111111010100100110001101000000000000000",
    );
    test_width(
        "1000000000000000",
        54,
        "000011100011010111111010100100110001101000000000000000",
        "0b0011100011010111111010100100110001101000000000000000",
    );
    test_width("-1000", 0, "-1111101000", "-0b1111101000");
    test_width("-1000", 1, "-1111101000", "-0b1111101000");
    test_width("-1000", 10, "-1111101000", "-0b1111101000");
    test_width("-1000", 13, "-001111101000", "-0b1111101000");
    test_width("-1000", 15, "-00001111101000", "-0b001111101000");
    test_width(
        "-1000000000000000",
        0,
        "-11100011010111111010100100110001101000000000000000",
        "-0b11100011010111111010100100110001101000000000000000",
    );
    test_width(
        "-1000000000000000",
        1,
        "-11100011010111111010100100110001101000000000000000",
        "-0b11100011010111111010100100110001101000000000000000",
    );
    test_width(
        "-1000000000000000",
        53,
        "-0011100011010111111010100100110001101000000000000000",
        "-0b11100011010111111010100100110001101000000000000000",
    );
    test_width(
        "-1000000000000000",
        55,
        "-000011100011010111111010100100110001101000000000000000",
        "-0b0011100011010111111010100100110001101000000000000000",
    );
}

#[test]
fn to_binary_string_properties() {
    integer_gen().test_properties(|x| {
        let s = x.to_binary_string();
        let prefixed_s = if x < 0 {
            "-0b".to_owned() + &s[1..]
        } else {
            "0b".to_owned() + &s
        };
        assert_eq!(format!("{x:#b}"), prefixed_s);
        assert_eq!(format!("{x:00b}"), s);
        assert_eq!(format!("{x:#00b}"), prefixed_s);
        assert_eq!(x.to_string_base(2), s);
        let num_x = BigInt::from(&x);
        assert_eq!(num_x.to_binary_string(), s);
        assert_eq!(format!("{num_x:#b}"), prefixed_s);
        let rug_x = rug::Integer::from(&x);
        assert_eq!(rug_x.to_binary_string(), s);
        assert_eq!(format!("{rug_x:#b}"), prefixed_s);
        assert!(string_is_subset(&s, "-01"));
        if x != 0 {
            assert!(!s.starts_with('0'));
        }
    });

    integer_unsigned_pair_gen_var_2().test_properties(|(x, width)| {
        let s = x.to_binary_string();
        let s_padded = format!("{x:0width$b}");
        test_padding(&s, &s_padded, width);
        assert_eq!(
            format!("{:0width$b}", BigInt::from(&x), width = width),
            s_padded
        );
        assert_eq!(
            format!("{:0width$b}", rug::Integer::from(&x), width = width),
            s_padded
        );

        let s_padded = format!("{x:#0width$b}");
        assert_eq!(
            format!("{:#0width$b}", BigInt::from(&x), width = width),
            s_padded
        );
        assert_eq!(
            format!("{:#0width$b}", rug::Integer::from(&x), width = width),
            s_padded
        );
    });

    signed_gen_var_2::<SignedLimb>().test_properties(|x| {
        assert_eq!(Integer::from(x).to_binary_string(), x.to_binary_string());
        assert_eq!(format!("{:#b}", Integer::from(x)), format!("{x:#b}"));
    });

    signed_unsigned_pair_gen_var_7::<SignedLimb, usize>().test_properties(|(x, width)| {
        assert_eq!(
            format!("{:0width$b}", Integer::from(x), width = width),
            format!("{x:0width$b}")
        );
        assert_eq!(
            format!("{:#0width$b}", Integer::from(x), width = width),
            format!("{x:#0width$b}")
        );
    });
}

#[test]
pub fn test_to_octal_string() {
    fn test(u: &str, out: &str, out_prefixed: &str) {
        let x = Integer::from_str(u).unwrap();
        assert_eq!(x.to_octal_string(), out);
        assert_eq!(format!("{x:00o}"), out);
        assert_eq!(format!("{x:#o}"), out_prefixed);
    }
    test("0", "0", "0o0");
    test("2", "2", "0o2");
    test("123", "173", "0o173");
    test("1000", "1750", "0o1750");
    test("1000000", "3641100", "0o3641100");
    test(
        "1000000000000000",
        "34327724461500000",
        "0o34327724461500000",
    );
    test("-2", "-2", "-0o2");
    test("-123", "-173", "-0o173");
    test("-1000", "-1750", "-0o1750");
    test("-1000000", "-3641100", "-0o3641100");
    test(
        "-1000000000000000",
        "-34327724461500000",
        "-0o34327724461500000",
    );

    fn test_width(u: &str, width: usize, out: &str, out_prefixed: &str) {
        let x = Integer::from_str(u).unwrap();
        let s = x.to_octal_string();
        assert_eq!(format!("{x:0width$o}"), out);
        assert_eq!(format!("{x:#0width$o}"), out_prefixed);
        test_padding(&s, out, width);
    }
    test_width("0", 0, "0", "0o0");
    test_width("0", 1, "0", "0o0");
    test_width("0", 2, "00", "0o0");
    test_width("0", 3, "000", "0o0");
    test_width("0", 4, "0000", "0o00");
    test_width("0", 5, "00000", "0o000");
    test_width("1000", 0, "1750", "0o1750");
    test_width("1000", 1, "1750", "0o1750");
    test_width("1000", 4, "1750", "0o1750");
    test_width("1000", 6, "001750", "0o1750");
    test_width("1000", 8, "00001750", "0o001750");
    test_width(
        "1000000000000000",
        0,
        "34327724461500000",
        "0o34327724461500000",
    );
    test_width(
        "1000000000000000",
        1,
        "34327724461500000",
        "0o34327724461500000",
    );
    test_width(
        "1000000000000000",
        19,
        "0034327724461500000",
        "0o34327724461500000",
    );
    test_width(
        "1000000000000000",
        21,
        "000034327724461500000",
        "0o0034327724461500000",
    );
    test_width("-1000", 0, "-1750", "-0o1750");
    test_width("-1000", 1, "-1750", "-0o1750");
    test_width("-1000", 4, "-1750", "-0o1750");
    test_width("-1000", 7, "-001750", "-0o1750");
    test_width("-1000", 9, "-00001750", "-0o001750");
    test_width(
        "-1000000000000000",
        0,
        "-34327724461500000",
        "-0o34327724461500000",
    );
    test_width(
        "-1000000000000000",
        1,
        "-34327724461500000",
        "-0o34327724461500000",
    );
    test_width(
        "-1000000000000000",
        20,
        "-0034327724461500000",
        "-0o34327724461500000",
    );
    test_width(
        "-1000000000000000",
        22,
        "-000034327724461500000",
        "-0o0034327724461500000",
    );
}

#[test]
fn to_octal_string_properties() {
    integer_gen().test_properties(|x| {
        let s = x.to_octal_string();
        let prefixed_s = if x < 0 {
            "-0o".to_owned() + &s[1..]
        } else {
            "0o".to_owned() + &s
        };
        assert_eq!(format!("{x:#o}"), prefixed_s);
        assert_eq!(format!("{x:00o}"), s);
        assert_eq!(format!("{x:#00o}"), prefixed_s);
        assert_eq!(x.to_string_base(8), s);
        let num_x = BigInt::from(&x);
        assert_eq!(num_x.to_octal_string(), s);
        assert_eq!(format!("{num_x:#o}"), prefixed_s);
        let rug_x = rug::Integer::from(&x);
        assert_eq!(rug_x.to_octal_string(), s);
        assert_eq!(format!("{rug_x:#o}"), prefixed_s);
        assert!(string_is_subset(&s, "-01234567"));
        if x != 0 {
            assert!(!s.starts_with('0'));
        }
    });

    integer_unsigned_pair_gen_var_2().test_properties(|(x, width)| {
        let s = x.to_octal_string();
        let s_padded = format!("{x:0width$o}");
        test_padding(&s, &s_padded, width);
        assert_eq!(
            format!("{:0width$o}", BigInt::from(&x), width = width),
            s_padded
        );
        assert_eq!(
            format!("{:0width$o}", rug::Integer::from(&x), width = width),
            s_padded
        );

        let s_padded = format!("{x:#0width$o}");
        assert_eq!(
            format!("{:#0width$o}", BigInt::from(&x), width = width),
            s_padded
        );
        assert_eq!(
            format!("{:#0width$o}", rug::Integer::from(&x), width = width),
            s_padded
        );
    });

    signed_gen_var_2::<SignedLimb>().test_properties(|x| {
        assert_eq!(Integer::from(x).to_octal_string(), x.to_octal_string());
        assert_eq!(format!("{:#o}", Integer::from(x)), format!("{x:#o}"));
    });

    signed_unsigned_pair_gen_var_7::<SignedLimb, usize>().test_properties(|(x, width)| {
        assert_eq!(
            format!("{:0width$o}", Integer::from(x), width = width),
            format!("{x:0width$o}")
        );
        assert_eq!(
            format!("{:#0width$o}", Integer::from(x), width = width),
            format!("{x:#0width$o}")
        );
    });
}

#[test]
pub fn test_to_lower_hex_string() {
    fn test(u: &str, out: &str, out_prefixed: &str) {
        let x = Integer::from_str(u).unwrap();
        assert_eq!(x.to_lower_hex_string(), out);
        assert_eq!(format!("{x:00x}"), out);
        assert_eq!(format!("{x:#x}"), out_prefixed);
    }
    test("0", "0", "0x0");
    test("2", "2", "0x2");
    test("123", "7b", "0x7b");
    test("1000", "3e8", "0x3e8");
    test("1000000", "f4240", "0xf4240");
    test("1000000000000000", "38d7ea4c68000", "0x38d7ea4c68000");
    test("-2", "-2", "-0x2");
    test("-123", "-7b", "-0x7b");
    test("-1000", "-3e8", "-0x3e8");
    test("-1000000", "-f4240", "-0xf4240");
    test("-1000000000000000", "-38d7ea4c68000", "-0x38d7ea4c68000");

    fn test_width(u: &str, width: usize, out: &str, out_prefixed: &str) {
        let x = Integer::from_str(u).unwrap();
        let s = x.to_lower_hex_string();
        assert_eq!(format!("{x:0width$x}"), out);
        assert_eq!(format!("{x:#0width$x}"), out_prefixed);
        test_padding(&s, out, width);
    }
    test_width("0", 0, "0", "0x0");
    test_width("0", 1, "0", "0x0");
    test_width("0", 2, "00", "0x0");
    test_width("0", 3, "000", "0x0");
    test_width("0", 4, "0000", "0x00");
    test_width("0", 5, "00000", "0x000");
    test_width("1000", 0, "3e8", "0x3e8");
    test_width("1000", 1, "3e8", "0x3e8");
    test_width("1000", 3, "3e8", "0x3e8");
    test_width("1000", 5, "003e8", "0x3e8");
    test_width("1000", 7, "00003e8", "0x003e8");
    test_width("1000000000000000", 0, "38d7ea4c68000", "0x38d7ea4c68000");
    test_width("1000000000000000", 1, "38d7ea4c68000", "0x38d7ea4c68000");
    test_width("1000000000000000", 15, "0038d7ea4c68000", "0x38d7ea4c68000");
    test_width(
        "1000000000000000",
        17,
        "000038d7ea4c68000",
        "0x0038d7ea4c68000",
    );
    test_width("-1000", 0, "-3e8", "-0x3e8");
    test_width("-1000", 1, "-3e8", "-0x3e8");
    test_width("-1000", 3, "-3e8", "-0x3e8");
    test_width("-1000", 6, "-003e8", "-0x3e8");
    test_width("-1000", 8, "-00003e8", "-0x003e8");
    test_width("-1000000000000000", 0, "-38d7ea4c68000", "-0x38d7ea4c68000");
    test_width("-1000000000000000", 1, "-38d7ea4c68000", "-0x38d7ea4c68000");
    test_width(
        "-1000000000000000",
        16,
        "-0038d7ea4c68000",
        "-0x38d7ea4c68000",
    );
    test_width(
        "-1000000000000000",
        18,
        "-000038d7ea4c68000",
        "-0x0038d7ea4c68000",
    );
}

#[test]
pub fn test_to_upper_hex_string() {
    fn test(u: &str, out: &str, out_prefixed: &str) {
        let x = Integer::from_str(u).unwrap();
        assert_eq!(x.to_upper_hex_string(), out);
        assert_eq!(format!("{x:00X}"), out);
        assert_eq!(format!("{x:#X}"), out_prefixed);
    }
    test("0", "0", "0x0");
    test("2", "2", "0x2");
    test("123", "7B", "0x7B");
    test("1000", "3E8", "0x3E8");
    test("1000000", "F4240", "0xF4240");
    test("1000000000000000", "38D7EA4C68000", "0x38D7EA4C68000");
    test("-2", "-2", "-0x2");
    test("-123", "-7B", "-0x7B");
    test("-1000", "-3E8", "-0x3E8");
    test("-1000000", "-F4240", "-0xF4240");
    test("-1000000000000000", "-38D7EA4C68000", "-0x38D7EA4C68000");

    fn test_width(u: &str, width: usize, out: &str, out_prefixed: &str) {
        let x = Integer::from_str(u).unwrap();
        let s = x.to_upper_hex_string();
        assert_eq!(format!("{x:0width$X}"), out);
        assert_eq!(format!("{x:#0width$X}"), out_prefixed);
        test_padding(&s, out, width);
    }
    test_width("0", 0, "0", "0x0");
    test_width("0", 1, "0", "0x0");
    test_width("0", 2, "00", "0x0");
    test_width("0", 3, "000", "0x0");
    test_width("0", 4, "0000", "0x00");
    test_width("0", 5, "00000", "0x000");
    test_width("1000", 0, "3E8", "0x3E8");
    test_width("1000", 1, "3E8", "0x3E8");
    test_width("1000", 3, "3E8", "0x3E8");
    test_width("1000", 5, "003E8", "0x3E8");
    test_width("1000", 7, "00003E8", "0x003E8");
    test_width("1000000000000000", 0, "38D7EA4C68000", "0x38D7EA4C68000");
    test_width("1000000000000000", 1, "38D7EA4C68000", "0x38D7EA4C68000");
    test_width("1000000000000000", 15, "0038D7EA4C68000", "0x38D7EA4C68000");
    test_width(
        "1000000000000000",
        17,
        "000038D7EA4C68000",
        "0x0038D7EA4C68000",
    );
    test_width("-1000", 0, "-3E8", "-0x3E8");
    test_width("-1000", 1, "-3E8", "-0x3E8");
    test_width("-1000", 3, "-3E8", "-0x3E8");
    test_width("-1000", 6, "-003E8", "-0x3E8");
    test_width("-1000", 8, "-00003E8", "-0x003E8");
    test_width("-1000000000000000", 0, "-38D7EA4C68000", "-0x38D7EA4C68000");
    test_width("-1000000000000000", 1, "-38D7EA4C68000", "-0x38D7EA4C68000");
    test_width(
        "-1000000000000000",
        16,
        "-0038D7EA4C68000",
        "-0x38D7EA4C68000",
    );
    test_width(
        "-1000000000000000",
        18,
        "-000038D7EA4C68000",
        "-0x0038D7EA4C68000",
    );
}

#[test]
fn to_hex_string_properties() {
    integer_gen().test_properties(|x| {
        let s = x.to_lower_hex_string();
        let prefixed_s = if x < 0 {
            "-0x".to_owned() + &s[1..]
        } else {
            "0x".to_owned() + &s
        };
        assert_eq!(format!("{x:#x}"), prefixed_s);
        assert_eq!(x.to_upper_hex_string(), s.to_ascii_uppercase());
        assert_eq!(
            format!("{x:#X}"),
            if x < 0 {
                "-0x".to_owned() + &s[1..].to_ascii_uppercase()
            } else {
                "0x".to_owned() + &s.to_ascii_uppercase()
            }
        );
        assert_eq!(format!("{x:00x}"), s);
        assert_eq!(format!("{x:#00x}"), prefixed_s);
        assert_eq!(format!("{x:00X}"), s.to_ascii_uppercase());
        assert_eq!(
            format!("{x:#00X}"),
            if x < 0 {
                "-0x".to_owned() + &s[1..].to_ascii_uppercase()
            } else {
                "0x".to_owned() + &s.to_ascii_uppercase()
            }
        );
        assert_eq!(x.to_string_base(16), s);
        let num_x = BigInt::from(&x);
        assert_eq!(num_x.to_lower_hex_string(), s);
        assert_eq!(format!("{num_x:#x}"), prefixed_s);
        let rug_x = rug::Integer::from(&x);
        assert_eq!(rug_x.to_lower_hex_string(), s);
        assert_eq!(format!("{rug_x:#x}"), prefixed_s);
        assert!(string_is_subset(&s, "-0123456789abcdef"));
        if x != 0 {
            assert!(!s.starts_with('0'));
        }
    });

    integer_unsigned_pair_gen_var_2().test_properties(|(x, width)| {
        let s = x.to_lower_hex_string();
        let s_padded = format!("{x:0width$x}");
        test_padding(&s, &s_padded, width);
        assert_eq!(
            format!("{:0width$x}", BigInt::from(&x), width = width),
            s_padded
        );
        assert_eq!(
            format!("{:0width$x}", rug::Integer::from(&x), width = width),
            s_padded
        );

        let s_padded = format!("{x:#0width$x}");
        assert_eq!(
            format!("{:#0width$x}", BigInt::from(&x), width = width),
            s_padded
        );
        assert_eq!(
            format!("{:#0width$x}", rug::Integer::from(&x), width = width),
            s_padded
        );

        let s = x.to_upper_hex_string();
        let s_padded_upper = format!("{x:0width$X}");
        assert_eq!(s_padded_upper, format!("{x:0width$x}").to_ascii_uppercase());
        let s_padded = s_padded_upper;
        test_padding(&s, &s_padded, width);
        assert_eq!(
            format!("{:0width$X}", BigInt::from(&x), width = width),
            s_padded
        );
        assert_eq!(
            format!("{:0width$X}", rug::Integer::from(&x), width = width),
            s_padded
        );

        let s_padded = format!("{x:#0width$X}");
        assert_eq!(
            format!("{:#0width$X}", BigInt::from(&x), width = width),
            s_padded
        );
        assert_eq!(
            format!("{:#0width$X}", rug::Integer::from(&x), width = width),
            s_padded
        );
    });

    signed_gen_var_2::<SignedLimb>().test_properties(|x| {
        assert_eq!(
            Integer::from(x).to_lower_hex_string(),
            x.to_lower_hex_string()
        );
        assert_eq!(
            Integer::from(x).to_upper_hex_string(),
            x.to_upper_hex_string()
        );
        assert_eq!(format!("{:#x}", Integer::from(x)), format!("{x:#x}"));
        assert_eq!(format!("{:#X}", Integer::from(x)), format!("{x:#X}"));
    });

    signed_unsigned_pair_gen_var_7::<SignedLimb, usize>().test_properties(|(x, width)| {
        assert_eq!(
            format!("{:0width$x}", Integer::from(x), width = width),
            format!("{x:0width$x}")
        );
        assert_eq!(
            format!("{:0width$X}", Integer::from(x), width = width),
            format!("{x:0width$X}")
        );
        assert_eq!(
            format!("{:#0width$x}", Integer::from(x), width = width),
            format!("{x:#0width$x}")
        );
        assert_eq!(
            format!("{:#0width$X}", Integer::from(x), width = width),
            format!("{x:#0width$X}")
        );
    });
}

#[test]
pub fn test_to_string_base() {
    fn test(u: &str, base: u8, out: &str) {
        let x = Integer::from_str(u).unwrap();
        assert_eq!(x.to_string_base(base), out);
        assert_eq!(format!("{}", BaseFmtWrapper::new(&x, base)), out);
        assert_eq!(format!("{:?}", BaseFmtWrapper::new(&x, base)), out);
        assert_eq!(format!("{:00}", BaseFmtWrapper::new(&x, base)), out);
        assert_eq!(format!("{:00?}", BaseFmtWrapper::new(&x, base)), out);
    }
    test("0", 2, "0");
    test("0", 3, "0");
    test("0", 10, "0");
    test("0", 16, "0");
    test("0", 17, "0");
    test("2", 3, "2");
    test("2", 10, "2");
    test("2", 16, "2");
    test("2", 17, "2");
    test("123", 8, "173");
    test("1000000", 10, "1000000");
    test("1000000", 20, "65000");
    test("1000000", 36, "lfls");
    test("1000", 2, "1111101000");
    test("1000", 3, "1101001");
    test("1000", 4, "33220");
    test("1000", 10, "1000");
    test("1000", 20, "2a0");
    test("1000", 36, "rs");
    test(
        "1000000000000000",
        2,
        "11100011010111111010100100110001101000000000000000",
    );
    test("1000000000000000", 3, "11212010201001210101011021212001");
    test("1000000000000000", 4, "3203113322210301220000000");
    test("1000000000000000", 10, "1000000000000000");
    test("1000000000000000", 20, "4hd2a0000000");
    test("1000000000000000", 36, "9ugxnorjls");
    test("-2", 3, "-2");
    test("-2", 10, "-2");
    test("-2", 16, "-2");
    test("-2", 17, "-2");
    test("-123", 8, "-173");
    test("-1000000", 10, "-1000000");
    test("-1000000", 20, "-65000");
    test("-1000000", 36, "-lfls");
    test("-1000", 2, "-1111101000");
    test("-1000", 3, "-1101001");
    test("-1000", 4, "-33220");
    test("-1000", 10, "-1000");
    test("-1000", 20, "-2a0");
    test("-1000", 36, "-rs");
    test(
        "-1000000000000000",
        2,
        "-11100011010111111010100100110001101000000000000000",
    );
    test("-1000000000000000", 3, "-11212010201001210101011021212001");
    test("-1000000000000000", 4, "-3203113322210301220000000");
    test("-1000000000000000", 10, "-1000000000000000");
    test("-1000000000000000", 20, "-4hd2a0000000");
    test("-1000000000000000", 36, "-9ugxnorjls");

    fn test_width(u: &str, base: u8, width: usize, out: &str) {
        let x = Integer::from_str(u).unwrap();
        let s = x.to_string_base(base);
        assert_eq!(
            format!("{:0width$}", BaseFmtWrapper::new(&x, base), width = width),
            out
        );
        assert_eq!(
            format!("{:0width$?}", BaseFmtWrapper::new(&x, base), width = width),
            out
        );
        test_padding(&s, out, width);
    }
    test_width("0", 2, 0, "0");
    test_width("0", 2, 1, "0");
    test_width("0", 2, 2, "00");
    test_width("0", 2, 5, "00000");
    test_width("1000000", 36, 0, "lfls");
    test_width("1000000", 36, 1, "lfls");
    test_width("1000000", 36, 2, "lfls");
    test_width("1000000", 36, 3, "lfls");
    test_width("1000000", 36, 4, "lfls");
    test_width("1000000", 36, 5, "0lfls");
    test_width("1000000", 36, 6, "00lfls");
    test_width("1000000000000000", 36, 0, "9ugxnorjls");
    test_width("1000000000000000", 36, 1, "9ugxnorjls");
    test_width("1000000000000000", 36, 10, "9ugxnorjls");
    test_width("1000000000000000", 36, 11, "09ugxnorjls");
    test_width("1000000000000000", 36, 20, "00000000009ugxnorjls");
    test_width("-1000000", 36, 0, "-lfls");
    test_width("-1000000", 36, 1, "-lfls");
    test_width("-1000000", 36, 2, "-lfls");
    test_width("-1000000", 36, 3, "-lfls");
    test_width("-1000000", 36, 4, "-lfls");
    test_width("-1000000", 36, 5, "-lfls");
    test_width("-1000000", 36, 6, "-0lfls");
    test_width("-1000000", 36, 7, "-00lfls");
    test_width("-1000000000000000", 36, 0, "-9ugxnorjls");
    test_width("-1000000000000000", 36, 1, "-9ugxnorjls");
    test_width("-1000000000000000", 36, 10, "-9ugxnorjls");
    test_width("-1000000000000000", 36, 11, "-9ugxnorjls");
    test_width("-1000000000000000", 36, 12, "-09ugxnorjls");
    test_width("-1000000000000000", 36, 21, "-00000000009ugxnorjls");
}

#[test]
fn to_string_base_fail() {
    assert_panic!(Integer::from(10).to_string_base(0));
    assert_panic!(Integer::from(10).to_string_base(1));
    assert_panic!(Integer::from(10).to_string_base(37));
    assert_panic!(Integer::from(10).to_string_base(100));
    assert_panic!(format!("{}", BaseFmtWrapper::new(&Integer::from(10), 0)));
    assert_panic!(format!("{}", BaseFmtWrapper::new(&Integer::from(10), 1)));
    assert_panic!(format!("{}", BaseFmtWrapper::new(&Integer::from(10), 37)));
    assert_panic!(format!("{}", BaseFmtWrapper::new(&Integer::from(10), 100)));
}

#[test]
fn to_string_base_properties() {
    integer_unsigned_pair_gen_var_1().test_properties(|(x, base)| {
        let s = x.to_string_base(base);
        assert_eq!(format!("{}", BaseFmtWrapper::new(&x, base)), s);
        assert_eq!(format!("{:?}", BaseFmtWrapper::new(&x, base)), s);
        assert_eq!(format!("{:00}", BaseFmtWrapper::new(&x, base)), s);
        assert_eq!(format!("{:00?}", BaseFmtWrapper::new(&x, base)), s);
        assert_eq!(x.to_string_base_upper(base), s.to_uppercase());
        assert_eq!(Integer::from_string_base(base, &s).unwrap(), x);
        assert!(string_is_subset(
            &s,
            "-0123456789abcdefghijklmnopqrstuvwxyz"
        ));
        if x != 0 {
            assert!(!s.starts_with('0'));
        }
    });

    integer_gen().test_properties(|x| {
        assert_eq!(x.to_string_base(10), x.to_string());
        assert_eq!(x.to_string_base(2), x.to_binary_string());
        assert_eq!(x.to_string_base(8), x.to_octal_string());
        assert_eq!(x.to_string_base(16), x.to_lower_hex_string());
    });

    unsigned_gen_var_8().test_properties(|base| {
        assert_eq!(Integer::ZERO.to_string_base(base), "0");
        assert_eq!(Integer::ONE.to_string_base(base), "1");
        assert_eq!(Integer::NEGATIVE_ONE.to_string_base(base), "-1");
        assert_eq!(Integer::from(base).to_string_base(base), "10");
    });

    integer_unsigned_unsigned_triple_gen_var_1().test_properties(|(x, base, width)| {
        let fx = BaseFmtWrapper::new(&x, base);
        let s = x.to_string_base(base);
        let s_padded = format!("{fx:0width$}");
        assert_eq!(format!("{fx:0width$?}"), s_padded);
        assert_eq!(Integer::from_string_base(base, &s).unwrap(), x);
        assert!(string_is_subset(
            &s_padded,
            "-0123456789abcdefghijklmnopqrstuvwxyz"
        ));
        test_padding(&s, &s_padded, width);
    });

    integer_unsigned_pair_gen_var_2().test_properties(|(x, width)| {
        assert_eq!(
            format!("{:0width$}", BaseFmtWrapper::new(&x, 10), width = width),
            format!("{x:0width$}")
        );
        assert_eq!(
            format!("{:0width$}", BaseFmtWrapper::new(&x, 2), width = width),
            format!("{x:0width$b}")
        );
        assert_eq!(
            format!("{:0width$}", BaseFmtWrapper::new(&x, 8), width = width),
            format!("{x:0width$o}")
        );
        assert_eq!(
            format!("{:0width$}", BaseFmtWrapper::new(&x, 16), width = width),
            format!("{x:0width$x}")
        );
    });

    unsigned_pair_gen_var_9::<usize, u8>().test_properties(|(width, base)| {
        let s = format!(
            "{:0width$}",
            BaseFmtWrapper::new(&Integer::ZERO, base),
            width = width
        );
        assert_eq!(repeat_n('0', max(1, width)).collect::<String>(), s);
    });

    signed_unsigned_unsigned_triple_gen_var_3::<SignedLimb, u8, usize>().test_properties(
        |(x, base, width)| {
            assert_eq!(
                format!(
                    "{:0width$}",
                    BaseFmtWrapper::new(&Integer::from(x), base),
                    width = width
                ),
                format!(
                    "{:0width$}",
                    BaseBaseFmtWrapper::new(x, base),
                    width = width
                ),
            );
        },
    );
}

#[test]
pub fn test_to_string_base_upper() {
    fn test(u: &str, base: u8, out: &str) {
        let x = Integer::from_str(u).unwrap();
        assert_eq!(x.to_string_base_upper(base), out);
        assert_eq!(format!("{:#}", BaseFmtWrapper::new(&x, base)), out);
        assert_eq!(format!("{:#?}", BaseFmtWrapper::new(&x, base)), out);
        assert_eq!(format!("{:#00}", BaseFmtWrapper::new(&x, base)), out);
        assert_eq!(format!("{:#00?}", BaseFmtWrapper::new(&x, base)), out);
    }
    test("0", 2, "0");
    test("0", 3, "0");
    test("0", 10, "0");
    test("0", 16, "0");
    test("0", 17, "0");
    test("2", 3, "2");
    test("2", 10, "2");
    test("2", 16, "2");
    test("2", 17, "2");
    test("123", 8, "173");
    test("1000000", 10, "1000000");
    test("1000000", 20, "65000");
    test("1000000", 36, "LFLS");
    test("1000", 2, "1111101000");
    test("1000", 3, "1101001");
    test("1000", 4, "33220");
    test("1000", 10, "1000");
    test("1000", 20, "2A0");
    test("1000", 36, "RS");
    test(
        "1000000000000000",
        2,
        "11100011010111111010100100110001101000000000000000",
    );
    test("1000000000000000", 3, "11212010201001210101011021212001");
    test("1000000000000000", 4, "3203113322210301220000000");
    test("1000000000000000", 10, "1000000000000000");
    test("1000000000000000", 20, "4HD2A0000000");
    test("1000000000000000", 36, "9UGXNORJLS");
    test("-2", 3, "-2");
    test("-2", 10, "-2");
    test("-2", 16, "-2");
    test("-2", 17, "-2");
    test("-123", 8, "-173");
    test("-1000000", 10, "-1000000");
    test("-1000000", 20, "-65000");
    test("-1000000", 36, "-LFLS");
    test("-1000", 2, "-1111101000");
    test("-1000", 3, "-1101001");
    test("-1000", 4, "-33220");
    test("-1000", 10, "-1000");
    test("-1000", 20, "-2A0");
    test("-1000", 36, "-RS");
    test(
        "-1000000000000000",
        2,
        "-11100011010111111010100100110001101000000000000000",
    );
    test("-1000000000000000", 3, "-11212010201001210101011021212001");
    test("-1000000000000000", 4, "-3203113322210301220000000");
    test("-1000000000000000", 10, "-1000000000000000");
    test("-1000000000000000", 20, "-4HD2A0000000");
    test("-1000000000000000", 36, "-9UGXNORJLS");

    fn test_width(u: &str, base: u8, width: usize, out: &str) {
        let x = Integer::from_str(u).unwrap();
        let s = x.to_string_base_upper(base);
        assert_eq!(
            format!("{:#0width$}", BaseFmtWrapper::new(&x, base), width = width),
            out
        );
        assert_eq!(
            format!("{:#0width$?}", BaseFmtWrapper::new(&x, base), width = width),
            out
        );
        test_padding(&s, out, width);
    }
    test_width("0", 2, 0, "0");
    test_width("0", 2, 1, "0");
    test_width("0", 2, 2, "00");
    test_width("0", 2, 5, "00000");
    test_width("1000000", 36, 0, "LFLS");
    test_width("1000000", 36, 1, "LFLS");
    test_width("1000000", 36, 2, "LFLS");
    test_width("1000000", 36, 3, "LFLS");
    test_width("1000000", 36, 4, "LFLS");
    test_width("1000000", 36, 5, "0LFLS");
    test_width("1000000", 36, 6, "00LFLS");
    test_width("1000000000000000", 36, 0, "9UGXNORJLS");
    test_width("1000000000000000", 36, 1, "9UGXNORJLS");
    test_width("1000000000000000", 36, 10, "9UGXNORJLS");
    test_width("1000000000000000", 36, 11, "09UGXNORJLS");
    test_width("1000000000000000", 36, 20, "00000000009UGXNORJLS");
    test_width("-1000000", 36, 0, "-LFLS");
    test_width("-1000000", 36, 1, "-LFLS");
    test_width("-1000000", 36, 2, "-LFLS");
    test_width("-1000000", 36, 3, "-LFLS");
    test_width("-1000000", 36, 4, "-LFLS");
    test_width("-1000000", 36, 5, "-LFLS");
    test_width("-1000000", 36, 6, "-0LFLS");
    test_width("-1000000", 36, 7, "-00LFLS");
    test_width("-1000000000000000", 36, 0, "-9UGXNORJLS");
    test_width("-1000000000000000", 36, 1, "-9UGXNORJLS");
    test_width("-1000000000000000", 36, 10, "-9UGXNORJLS");
    test_width("-1000000000000000", 36, 11, "-9UGXNORJLS");
    test_width("-1000000000000000", 36, 12, "-09UGXNORJLS");
    test_width("-1000000000000000", 36, 21, "-00000000009UGXNORJLS");
}

#[test]
fn to_string_base_upper_fail() {
    assert_panic!(Integer::from(10).to_string_base_upper(0));
    assert_panic!(Integer::from(10).to_string_base_upper(1));
    assert_panic!(Integer::from(10).to_string_base_upper(37));
    assert_panic!(Integer::from(10).to_string_base_upper(100));
    assert_panic!(format!("{:#}", BaseFmtWrapper::new(&Integer::from(10), 0)));
    assert_panic!(format!("{:#}", BaseFmtWrapper::new(&Integer::from(10), 1)));
    assert_panic!(format!("{:#}", BaseFmtWrapper::new(&Integer::from(10), 37)));
    assert_panic!(format!(
        "{:#}",
        BaseFmtWrapper::new(&Integer::from(10), 100)
    ));
}

#[test]
fn to_string_base_upper_properties() {
    integer_unsigned_pair_gen_var_1().test_properties(|(x, base)| {
        let s = x.to_string_base_upper(base);
        assert_eq!(format!("{:#}", BaseFmtWrapper::new(&x, base)), s);
        assert_eq!(format!("{:#?}", BaseFmtWrapper::new(&x, base)), s);
        assert_eq!(format!("{:#00}", BaseFmtWrapper::new(&x, base)), s);
        assert_eq!(x.to_string_base(base), s.to_lowercase());
        assert_eq!(Integer::from_string_base(base, &s).unwrap(), x);
        assert!(string_is_subset(
            &s,
            "-0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ"
        ));
        if x != 0 {
            assert!(!s.starts_with('0'));
        }
    });

    integer_gen().test_properties(|x| {
        assert_eq!(x.to_string_base_upper(10), x.to_string());
        assert_eq!(x.to_string_base_upper(2), x.to_binary_string());
        assert_eq!(x.to_string_base_upper(8), x.to_octal_string());
        assert_eq!(x.to_string_base_upper(16), x.to_upper_hex_string());
    });

    unsigned_gen_var_8().test_properties(|base| {
        assert_eq!(Integer::ZERO.to_string_base_upper(base), "0");
        assert_eq!(Integer::ONE.to_string_base_upper(base), "1");
        assert_eq!(Integer::NEGATIVE_ONE.to_string_base_upper(base), "-1");
        assert_eq!(Integer::from(base).to_string_base_upper(base), "10");
    });

    integer_unsigned_unsigned_triple_gen_var_1().test_properties(|(x, base, width)| {
        let fx = BaseFmtWrapper::new(&x, base);
        let s = x.to_string_base_upper(base);
        let s_padded = format!("{fx:#0width$}");
        assert_eq!(format!("{fx:#0width$?}"), s_padded);
        assert_eq!(Integer::from_string_base(base, &s).unwrap(), x);
        assert!(string_is_subset(
            &s_padded,
            "-0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ"
        ));
        test_padding(&s, &s_padded, width);
    });

    integer_unsigned_pair_gen_var_2().test_properties(|(x, width)| {
        assert_eq!(
            format!("{:#0width$}", BaseFmtWrapper::new(&x, 10), width = width),
            format!("{x:0width$}")
        );
        assert_eq!(
            format!("{:#0width$}", BaseFmtWrapper::new(&x, 2), width = width),
            format!("{x:0width$b}")
        );
        assert_eq!(
            format!("{:#0width$}", BaseFmtWrapper::new(&x, 8), width = width),
            format!("{x:0width$o}")
        );
        assert_eq!(
            format!("{:#0width$}", BaseFmtWrapper::new(&x, 16), width = width),
            format!("{x:0width$X}")
        );
    });

    unsigned_pair_gen_var_9::<usize, u8>().test_properties(|(width, base)| {
        let s = format!(
            "{:#0width$}",
            BaseFmtWrapper::new(&Integer::ZERO, base),
            width = width
        );
        assert_eq!(repeat_n('0', max(1, width)).collect::<String>(), s);
    });

    signed_unsigned_unsigned_triple_gen_var_3::<SignedLimb, u8, usize>().test_properties(
        |(x, base, width)| {
            assert_eq!(
                format!(
                    "{:#0width$}",
                    BaseFmtWrapper::new(&Integer::from(x), base),
                    width = width
                ),
                format!(
                    "{:#0width$}",
                    BaseBaseFmtWrapper::new(x, base),
                    width = width
                ),
            );
        },
    );
}
