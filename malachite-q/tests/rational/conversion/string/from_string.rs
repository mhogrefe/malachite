// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::assert_panic;
use malachite_base::num::conversion::traits::{FromStringBase, ToStringBase};
use malachite_base::test_util::generators::common::GenConfig;
use malachite_base::test_util::generators::string_gen;
use malachite_nz::integer::Integer;
use malachite_nz::test_util::generators::integer_gen;
use malachite_q::Rational;
use malachite_q::test_util::generators::{rational_unsigned_pair_gen_var_10, string_gen_var_12};
use num::BigRational;
use std::panic::catch_unwind;
use std::str::FromStr;

#[test]
fn test_from_str() {
    let test_ok = |s, n| {
        assert_eq!(Rational::from_str(s).unwrap().to_string(), n);
        assert_eq!(BigRational::from_str(s).unwrap().to_string(), n);
        assert_eq!(rug::Rational::from_str(s).unwrap().to_string(), n);
    };
    test_ok("0", "0");
    test_ok("-0", "0");
    test_ok("123456", "123456");
    test_ok("1000000000000000000000000", "1000000000000000000000000");
    test_ok("-123456", "-123456");
    test_ok("-1000000000000000000000000", "-1000000000000000000000000");
    test_ok("01/02", "1/2");
    test_ok("3/21", "1/7");

    let test_err = |s, rug_err| {
        assert!(Rational::from_str(s).is_err());
        assert!(BigRational::from_str(s).is_err());
        let rn = rug::Rational::from_str(s);
        assert_eq!(rn.is_err() || rn.unwrap() < 0, rug_err);
    };
    test_err("12A", true);
    test_err(" 10", false);
    test_err("1.0", true);
    test_err("$%^", true);
    test_err("", true);
    test_err("-", true);
    test_err("1/0", true);
    test_err("/1", true);
    test_err("--0", true);
    test_err("-+0", true);
    test_err("+-0", true);
    test_err("++0", true);
    test_err("--1", true);
    test_err("-+1", true);
    test_err("+-1", true);
    test_err("++1", true);
}

#[allow(unused_must_use)]
#[test]
fn from_str_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 64);
    string_gen().test_properties_with_config(&config, |s| {
        Rational::from_str(&s);
    });

    string_gen_var_12().test_properties(|s| {
        let n = Rational::from_str(&s).unwrap();
        assert_eq!(BigRational::from_str(&s).unwrap(), BigRational::from(&n));
        assert_eq!(
            rug::Rational::from_str(&s).unwrap(),
            rug::Rational::from(&n)
        );
    });

    integer_gen().test_properties(|x| {
        let s = x.to_string();
        assert_eq!(
            Rational::from_str(&s).unwrap(),
            Integer::from_str(&s).unwrap()
        );
    });
}

// The string must be syntactically valid with a nonzero denominator; the result is not
// canonicalized until __gmpq_canonicalize (which would divide by zero on a zero denominator).
pub fn gmp_q_from_string_base(base: i32, s: &str) -> rug::Rational {
    unsafe extern "C" {
        fn __gmpq_canonicalize(rop: *mut core::ffi::c_void);
        fn __gmpq_set_str(
            rop: *mut core::ffi::c_void,
            s: *const core::ffi::c_char,
            base: core::ffi::c_int,
        ) -> core::ffi::c_int;
    }
    let cs = std::ffi::CString::new(s).unwrap();
    let mut x = rug::Rational::new();
    unsafe {
        let rop = x.as_raw_mut() as *mut core::ffi::c_void;
        assert_eq!(__gmpq_set_str(rop, cs.as_ptr(), base), 0, "{s}");
        __gmpq_canonicalize(rop);
    }
    x
}

#[test]
fn test_from_string_base() {
    fn test_ok(base: u8, s: &str, out: &str) {
        assert_eq!(
            Rational::from_string_base(base, s).unwrap().to_string(),
            out
        );
    }
    test_ok(10, "0", "0");
    test_ok(10, "22/7", "22/7");
    test_ok(10, "-22/7", "-22/7");
    test_ok(10, "01/02", "1/2");
    test_ok(10, "3/21", "1/7");
    test_ok(10, "-00123456", "-123456");
    test_ok(16, "-ff/7", "-255/7");
    // for bases up to 36, parsing is case-insensitive
    test_ok(36, "G8", "584");
    test_ok(36, "g8", "584");
    // above base 36, the uppercase and lowercase letters are distinct digits
    test_ok(62, "G8/z", "1000/61");
    test_ok(62, "g8/z", "2612/61");
    test_ok(62, "+G8/z", "1000/61");

    fn test_err(base: u8, s: &str) {
        assert!(Rational::from_string_base(base, s).is_none());
    }
    test_err(10, "");
    test_err(10, "a");
    test_err(10, "1/0");
    test_err(10, "/1");
    test_err(10, "1/");
    test_err(10, "--1");
    test_err(10, "-+1");
    test_err(10, "++1");
    test_err(10, "1/-2");
    test_err(10, "1/+-2");
    test_err(10, "2/2/2");
    // 'b' is the digit 37, out of range for base 37
    test_err(37, "b/2");
    // 'z' is the digit 61, out of range for base 61
    test_err(61, "z/2");
}

#[test]
fn from_string_base_fail() {
    assert_panic!(Rational::from_string_base(1, "0"));
    assert_panic!(Rational::from_string_base(63, "0"));
}

#[test]
fn from_string_base_properties() {
    rational_unsigned_pair_gen_var_10().test_properties(|(x, base)| {
        let s = x.to_string_base(base);
        assert_eq!(Rational::from_string_base(base, &s).unwrap(), x);
        assert_eq!(
            gmp_q_from_string_base(i32::from(base), &s),
            rug::Rational::from(&x)
        );
    });

    string_gen().test_properties(|s| {
        let ox = Rational::from_string_base(10, &s);
        assert_eq!(ox, Rational::from_str(&s).ok());
        if let Some(x) = ox {
            assert_eq!(
                Rational::from_string_base(10, &x.to_string_base(10)).unwrap(),
                x
            );
        }
    });

    string_gen_var_12().test_properties(|s| {
        assert_eq!(
            Rational::from_string_base(10, &s).unwrap(),
            Rational::from_str(&s).unwrap()
        );
    });
}
