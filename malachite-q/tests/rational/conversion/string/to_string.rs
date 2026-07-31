// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::assert_panic;
use malachite_base::num::conversion::traits::{FromStringBase, ToStringBase};
use malachite_base::strings::ToDebugString;
use malachite_base::strings::string_is_subset;
use malachite_nz::test_util::generators::integer_gen;
use malachite_q::Rational;
use malachite_q::test_util::generators::{rational_gen, rational_unsigned_pair_gen_var_10};
use num::BigRational;
use std::panic::catch_unwind;
use std::str::FromStr;

#[test]
pub fn test_to_string() {
    fn test(u: &str) {
        let x = Rational::from_str(u).unwrap();
        assert_eq!(x.to_string(), u);
        assert_eq!(x.to_debug_string(), u);
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
    test("99/100");
    test("101/100");
    test("22/7");
    test("-99/100");
    test("-101/100");
    test("-22/7");
}

#[test]
fn to_string_properties() {
    rational_gen().test_properties(|x| {
        let s = x.to_string();
        assert_eq!(x.to_debug_string(), s);
        assert_eq!(BigRational::from(&x).to_string(), s);
        assert_eq!(rug::Rational::from(&x).to_string(), s);
        assert!(string_is_subset(&s, "-/0123456789"));
        if x != 0 {
            assert!(!s.starts_with('0'));
        }
    });

    integer_gen().test_properties(|x| {
        assert_eq!(Rational::from(&x).to_string(), x.to_string());
    });
}

pub fn gmp_q_to_string_base(x: &rug::Rational, base: i32) -> String {
    unsafe extern "C" {
        fn __gmpq_get_str(
            buf: *mut core::ffi::c_char,
            base: core::ffi::c_int,
            op: *const core::ffi::c_void,
        ) -> *mut core::ffi::c_char;
    }
    let len = x.numer().significant_bits() + x.denom().significant_bits();
    let mut buf = vec![0u8; usize::try_from(len).unwrap() + 5];
    unsafe {
        __gmpq_get_str(
            buf.as_mut_ptr().cast(),
            base,
            x.as_raw() as *const core::ffi::c_void,
        );
    }
    let end = buf.iter().position(|&b| b == 0).unwrap();
    String::from_utf8_lossy(&buf[..end]).into_owned()
}

#[test]
fn test_to_string_base() {
    fn test(n: i64, d: i64, base: u8, out: &str) {
        let x = Rational::from_signeds(n, d);
        assert_eq!(x.to_string_base(base), out);
        let upper = x.to_string_base_upper(base);
        if base > 36 {
            // above base 36 there is only one alphabet, so the uppercase variant is the same
            assert_eq!(upper, out);
        } else {
            assert_eq!(upper, out.to_uppercase());
        }
        assert_eq!(
            gmp_q_to_string_base(&rug::Rational::from((n, d)), i32::from(base)),
            out
        );
    }
    test(0, 1, 62, "0");
    test(22, 7, 10, "22/7");
    test(-22, 7, 10, "-22/7");
    test(255, 7, 16, "ff/7");
    test(1000, 7, 36, "rs/7");
    test(1000, 7, 37, "R1/7");
    test(1000, 61, 61, "GO/10");
    test(1000, 61, 62, "G8/z");
    test(-1000, 61, 62, "-G8/z");
    test(3844, 1, 62, "100");
}

#[test]
fn to_string_base_fail() {
    assert_panic!(Rational::from_signeds(22, 7).to_string_base(1));
    assert_panic!(Rational::from_signeds(22, 7).to_string_base(63));
    assert_panic!(Rational::from_signeds(22, 7).to_string_base_upper(1));
    assert_panic!(Rational::from_signeds(22, 7).to_string_base_upper(63));
}

#[test]
fn to_string_base_properties() {
    rational_unsigned_pair_gen_var_10().test_properties(|(x, base)| {
        let s = x.to_string_base(base);
        assert_eq!(
            gmp_q_to_string_base(&rug::Rational::from(&x), i32::from(base)),
            s
        );
        assert_eq!(Rational::from_string_base(base, &s).unwrap(), x);
        let upper = x.to_string_base_upper(base);
        if base > 36 {
            assert_eq!(upper, s);
        } else {
            assert_eq!(upper, s.to_uppercase());
            // for bases up to 36, a negative base means uppercase digits in GMP
            assert_eq!(
                gmp_q_to_string_base(&rug::Rational::from(&x), -i32::from(base)),
                upper
            );
            assert_eq!(Rational::from_string_base(base, &upper).unwrap(), x);
        }
        assert!(string_is_subset(
            &s,
            "-/0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz"
        ));
        if x != 0 {
            assert!(!s.starts_with('0'));
        }
    });

    rational_gen().test_properties(|x| {
        assert_eq!(x.to_string_base(10), x.to_string());
    });
}
