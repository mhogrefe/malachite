// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::conversion::traits::{ExactFrom, FromStringBase, ToStringBase};
use malachite_base::test_util::generators::common::GenConfig;
use malachite_base::test_util::generators::exhaustive::valid_digit_chars;
use malachite_base::test_util::generators::{
    signed_gen, signed_unsigned_pair_gen_var_5, string_gen, string_gen_var_4, unsigned_gen_var_11,
    unsigned_pair_gen_var_19, unsigned_string_pair_gen_var_2, unsigned_string_pair_gen_var_3,
};
use malachite_nz::integer::Integer;
use malachite_nz::platform::SignedLimb;
use num::{BigInt, Num};
use rug;
use std::collections::HashMap;
use std::str::FromStr;

#[test]
fn test_from_str() {
    let test_ok = |s, n| {
        assert_eq!(Integer::from_str(s).unwrap().to_string(), n);
        assert_eq!(BigInt::from_str(s).unwrap().to_string(), n);
        assert_eq!(rug::Integer::from_str(s).unwrap().to_string(), n);
    };
    test_ok("0", "0");
    test_ok("-0", "0");
    test_ok("123456", "123456");
    test_ok("1000000000000000000000000", "1000000000000000000000000");
    test_ok("-123456", "-123456");
    test_ok("-1000000000000000000000000", "-1000000000000000000000000");

    let test_err = |s, rug_err| {
        assert!(Integer::from_str(s).is_err());
        assert!(BigInt::from_str(s).is_err());
        let rn = rug::Integer::from_str(s);
        assert_eq!(rn.is_err() || rn.unwrap() < 0, rug_err);
    };
    test_err("12A", true);
    test_err(" 10", false);
    test_err("1.0", true);
    test_err("$%^", true);
    test_err("", true);
    test_err("-", true);
    test_err("--0", true);
    test_err("-+0", true);
    test_err("+-0", true);
    test_err("++0", true);
    test_err("--1", true);
    test_err("-+1", true);
    test_err("+-1", true);
    test_err("++1", true);
}

#[test]
fn from_str_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 64);
    string_gen().test_properties_with_config(&config, |s| {
        let trimmed = s.strip_prefix('-').unwrap_or(&s);
        assert_eq!(
            Integer::from_str(&s).is_ok(),
            !trimmed.is_empty() && trimmed.chars().all(|c| c.is_ascii_digit()),
        );
    });

    string_gen_var_4().test_properties(|s| {
        let n = Integer::from_str(&s).unwrap();
        let abs_s = s.strip_prefix('-').unwrap_or(&s);
        let trimmed = abs_s.trim_start_matches('0');
        let trimmed = if trimmed.is_empty() {
            "0".to_string()
        } else if s.starts_with('-') {
            "-".to_string() + trimmed
        } else {
            trimmed.to_string()
        };
        assert_eq!(n.to_string(), trimmed);
        assert_eq!(n, Integer::from_string_base(10, &s).unwrap());
        let mut with_zero = "0".to_string() + abs_s;
        if s.starts_with('-') {
            with_zero = "-".to_string() + &with_zero;
        }
        assert_eq!(Integer::from_str(&with_zero).unwrap(), n);

        assert_eq!(BigInt::from_str(&s).unwrap(), BigInt::from(&n));
        assert_eq!(rug::Integer::from_str(&s).unwrap(), rug::Integer::from(&n));
    });

    unsigned_gen_var_11().test_properties(|u| {
        let zeros = vec![b'0'; u];
        let zero_s = std::str::from_utf8(&zeros).unwrap();
        assert_eq!(Integer::from_str(zero_s).unwrap(), 0);
        assert_eq!(Integer::from_str(&("-".to_string() + zero_s)).unwrap(), 0);
    });

    signed_gen::<SignedLimb>().test_properties(|u| {
        let s = u.to_string();
        assert_eq!(
            Integer::from_str(&s).unwrap(),
            Integer::from(SignedLimb::from_str(&s).unwrap())
        );
    });
}

#[test]
fn test_from_string_base() {
    let test_ok = |base, s, n| {
        assert_eq!(Integer::from_string_base(base, s).unwrap().to_string(), n);
        assert_eq!(
            BigInt::from_str_radix(s, u32::exact_from(base))
                .unwrap()
                .to_string(),
            n
        );
        assert_eq!(
            rug::Integer::from_str_radix(s, i32::exact_from(base))
                .unwrap()
                .to_string(),
            n
        );
    };
    test_ok(2, "0", "0");
    test_ok(10, "0", "0");
    test_ok(2, "101", "5");
    test_ok(10, "123456", "123456");
    test_ok(16, "deadbeef", "3735928559");
    test_ok(16, "DEADBEEF", "3735928559");
    test_ok(16, "deAdBeEf", "3735928559");
    test_ok(10, "1000000000000000000000000", "1000000000000000000000000");
    test_ok(2, "1000000000000000000000000", "16777216");
    test_ok(
        36,
        "1000000000000000000000000",
        "22452257707354557240087211123792674816",
    );
    test_ok(36, "helloworld", "1767707668033969");
    test_ok(2, "-0", "0");
    test_ok(10, "-0", "0");
    test_ok(2, "-101", "-5");
    test_ok(10, "-123456", "-123456");
    test_ok(16, "-deadbeef", "-3735928559");
    test_ok(16, "-DEADBEEF", "-3735928559");
    test_ok(16, "-deAdBeEf", "-3735928559");
    test_ok(
        10,
        "-1000000000000000000000000",
        "-1000000000000000000000000",
    );
    test_ok(2, "-1000000000000000000000000", "-16777216");
    test_ok(
        36,
        "-1000000000000000000000000",
        "-22452257707354557240087211123792674816",
    );
    test_ok(36, "-helloworld", "-1767707668033969");

    let test_err = |base, s, rug_err| {
        assert!(Integer::from_string_base(base, s).is_none());
        assert!(BigInt::from_str_radix(s, u32::exact_from(base)).is_err());
        assert_eq!(
            rug::Integer::from_str_radix(s, i32::exact_from(base)).is_err(),
            rug_err
        );
    };
    test_err(2, "123", true);
    test_err(10, "12A", true);
    test_err(35, " 10", false);
    test_err(35, "1.0", true);
    test_err(35, "$%^", true);
    test_err(35, "", true);
    test_err(35, "-", true);
}

#[test]
fn from_string_base_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 64);
    let mut digit_map = HashMap::new();
    unsigned_string_pair_gen_var_2().test_properties_with_config(&config, |(base, s)| {
        let abs_s = s.strip_prefix('-').unwrap_or(&s);
        let digits = digit_map
            .entry(base)
            .or_insert_with(|| valid_digit_chars(base));
        assert_eq!(
            Integer::from_string_base(base, &s).is_some(),
            !abs_s.is_empty() && abs_s.chars().all(|c| digits.contains(&c)),
        );
    });

    unsigned_string_pair_gen_var_3().test_properties(|(base, s)| {
        let n = Integer::from_string_base(base, &s).unwrap();
        let s_lo = s.to_lowercase();
        let abs_s = s_lo.strip_prefix('-').unwrap_or(&s_lo);
        let trimmed = abs_s.trim_start_matches('0');
        let trimmed = if trimmed.is_empty() {
            "0".to_string()
        } else if s.starts_with('-') {
            "-".to_string() + trimmed
        } else {
            trimmed.to_string()
        };
        assert_eq!(n.to_string_base(base), trimmed);
        let mut with_zero = "0".to_string() + abs_s;
        if s.starts_with('-') {
            with_zero = "-".to_string() + &with_zero;
        }
        assert_eq!(Integer::from_string_base(base, &with_zero).unwrap(), n);

        assert_eq!(
            BigInt::from_str_radix(&s, u32::from(base)).unwrap(),
            BigInt::from(&n)
        );
        assert_eq!(
            rug::Integer::from_str_radix(&s, i32::from(base)).unwrap(),
            rug::Integer::from(&n)
        );
    });

    unsigned_pair_gen_var_19().test_properties(|(u, base)| {
        let zeros = vec![b'0'; u];
        let zero_s = std::str::from_utf8(&zeros).unwrap();
        assert_eq!(Integer::from_string_base(base, zero_s).unwrap(), 0);
        assert_eq!(
            Integer::from_string_base(base, &("-".to_string() + zero_s)).unwrap(),
            0
        );
    });

    signed_unsigned_pair_gen_var_5::<SignedLimb, u8>().test_properties(|(i, base)| {
        let s = i.to_string_base(base);
        assert_eq!(
            Integer::from_string_base(base, &s).unwrap(),
            Integer::from(SignedLimb::from_string_base(base, &s).unwrap())
        );
    });
}
