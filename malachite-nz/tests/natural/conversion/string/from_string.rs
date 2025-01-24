// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::conversion::traits::{
    ExactFrom, FromStringBase, ToStringBase, WrappingFrom,
};
use malachite_base::test_util::generators::common::GenConfig;
use malachite_base::test_util::generators::exhaustive::valid_digit_chars;
use malachite_base::test_util::generators::{
    string_gen, string_gen_var_3, string_gen_var_5, string_gen_var_6, string_gen_var_7,
    unsigned_gen, unsigned_gen_var_11, unsigned_pair_gen_var_19, unsigned_pair_gen_var_8,
    unsigned_string_pair_gen_var_1, unsigned_string_pair_gen_var_2,
};
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use num::{BigUint, Num};
use rug;
use std::collections::HashMap;
use std::str::FromStr;

#[test]
fn test_from_str() {
    let test_ok = |s, n| {
        assert_eq!(Natural::from_str(s).unwrap().to_string(), n);
        assert_eq!(BigUint::from_str(s).unwrap().to_string(), n);
        assert_eq!(rug::Integer::from_str(s).unwrap().to_string(), n);
    };
    test_ok("0", "0");
    test_ok("123456", "123456");
    test_ok("1000000000000000000000000", "1000000000000000000000000");

    let test_err = |s, rug_err| {
        assert!(Natural::from_str(s).is_err());
        assert!(BigUint::from_str(s).is_err());
        let rn = rug::Integer::from_str(s);
        assert_eq!(rn.is_err() || rn.unwrap() < 0, rug_err);
    };
    test_err("12A", true);
    test_err(" 10", false);
    test_err("1.0", true);
    test_err("-5", true);
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
        assert_eq!(
            Natural::from_str(&s).is_ok(),
            !s.is_empty() && s.chars().all(|c| c.is_ascii_digit()),
        );
    });

    string_gen_var_3().test_properties(|s| {
        let n = Natural::from_str(&s).unwrap();
        let mut trimmed = s.trim_start_matches('0');
        if trimmed.is_empty() {
            trimmed = "0";
        }
        assert_eq!(n.to_string(), trimmed);
        assert_eq!(n, Natural::from_string_base(10, &s).unwrap());
        let with_zero = "0".to_string() + &s;
        assert_eq!(Natural::from_str(&with_zero).unwrap(), n);

        assert_eq!(BigUint::from_str(&s).unwrap(), BigUint::from(&n));
        assert_eq!(rug::Integer::from_str(&s).unwrap(), rug::Integer::from(&n));
    });

    unsigned_gen_var_11().test_properties(|u| {
        assert_eq!(
            Natural::from_str(std::str::from_utf8(&vec![b'0'; u]).unwrap()).unwrap(),
            0
        );
    });

    unsigned_gen::<Limb>().test_properties(|u| {
        let s = u.to_string();
        assert_eq!(
            Natural::from_str(&s).unwrap(),
            Natural::from(Limb::from_str(&s).unwrap())
        );
    });
}

#[test]
fn test_from_string_base() {
    let test_ok = |base, s, n| {
        assert_eq!(Natural::from_string_base(base, s).unwrap().to_string(), n);
        assert_eq!(
            BigUint::from_str_radix(s, u32::exact_from(base))
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

    let test_err = |base, s, rug_err| {
        assert!(Natural::from_string_base(base, s).is_none());
        assert!(BigUint::from_str_radix(s, u32::exact_from(base)).is_err());
        let rn = rug::Integer::from_str_radix(s, i32::exact_from(base));
        assert_eq!(rn.is_err() || rn.unwrap() < 0, rug_err);
    };
    test_err(2, "123", true);
    test_err(10, "12A", true);
    test_err(35, " 10", false);
    test_err(35, "1.0", true);
    test_err(35, "-5", true);
    test_err(35, "$%^", true);
    test_err(35, "", true);
    test_err(35, "-", true);
    test_err(16, "10000000z", true);
    test_err(16, "1000000000000000z", true);
}

#[test]
#[should_panic]
fn from_string_base_fail_1() {
    Natural::from_string_base(1, "0");
}

#[test]
#[should_panic]
fn from_string_base_fail_2() {
    Natural::from_string_base(0, "0");
}

fn from_string_base_helper(base: u8, s: &str) {
    let n = Natural::from_string_base(base, s).unwrap();
    let s_lo = s.to_lowercase();
    let mut trimmed = s_lo.trim_start_matches('0');
    if trimmed.is_empty() {
        trimmed = "0";
    }
    assert_eq!(n.to_string_base(base), trimmed);
    let with_zero = "0".to_string() + s;
    assert_eq!(Natural::from_string_base(base, &with_zero).unwrap(), n);

    assert_eq!(
        BigUint::from_str_radix(s, u32::from(base)).unwrap(),
        BigUint::from(&n)
    );
    assert_eq!(
        rug::Integer::from_str_radix(s, i32::from(base)).unwrap(),
        rug::Integer::from(&n)
    );
}

#[test]
fn from_string_base_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 64);
    let mut digit_map = HashMap::new();
    unsigned_string_pair_gen_var_2().test_properties_with_config(&config, |(base, s)| {
        let digits = digit_map
            .entry(base)
            .or_insert_with(|| valid_digit_chars(u8::wrapping_from(base)));
        assert_eq!(
            Natural::from_string_base(base, &s).is_some(),
            !s.is_empty() && s.chars().all(|c| digits.contains(&c)),
        );
    });

    unsigned_string_pair_gen_var_1().test_properties(|(base, s)| {
        from_string_base_helper(base, &s);
    });

    string_gen_var_5().test_properties(|s| {
        from_string_base_helper(2, &s);
    });

    string_gen_var_6().test_properties(|s| {
        from_string_base_helper(8, &s);
    });

    string_gen_var_7().test_properties(|s| {
        from_string_base_helper(16, &s);
    });

    unsigned_pair_gen_var_19().test_properties(|(u, base)| {
        assert_eq!(
            Natural::from_string_base(base, std::str::from_utf8(&vec![b'0'; u]).unwrap()).unwrap(),
            0
        );
    });

    unsigned_pair_gen_var_8::<Limb, u8>().test_properties(|(u, base)| {
        let s = u.to_string_base(base);
        assert_eq!(
            Natural::from_string_base(base, &s).unwrap(),
            Natural::from(Limb::from_string_base(base, &s).unwrap())
        );
    });
}
