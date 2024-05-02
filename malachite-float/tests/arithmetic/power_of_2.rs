// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{IsPowerOf2, PowerOf2};
use malachite_base::test_util::generators::{
    signed_gen_var_5, signed_unsigned_pair_gen_var_19, unsigned_gen_var_5,
};
use malachite_float::test_util::common::to_hex_string;
use malachite_float::{ComparableFloatRef, Float};

#[test]
fn test_power_of_2_prec() {
    let test = |i: i64, prec: u64, out: &str, out_hex: &str| {
        let p = Float::power_of_2_prec(i, prec);
        assert!(p.is_valid());

        assert_eq!(p.to_string(), out);
        assert_eq!(to_hex_string(&p), out_hex);
    };
    test(0, 1, "1.0", "0x1.0#1");
    test(0, 100, "1.0", "0x1.0000000000000000000000000#100");
    test(1, 1, "2.0", "0x2.0#1");
    test(1, 100, "2.0", "0x2.0000000000000000000000000#100");
    test(100, 1, "1.0e30", "0x1.0E+25#1");
    test(
        100,
        100,
        "1267650600228229401496703205376.0",
        "0x10000000000000000000000000.0#100",
    );
    test(-1, 1, "0.5", "0x0.8#1");
    test(-1, 100, "0.5", "0x0.8000000000000000000000000#100");
    test(-100, 1, "8.0e-31", "0x1.0E-25#1");
    test(
        -100,
        100,
        "7.88860905221011805411728565283e-31",
        "0x1.0000000000000000000000000E-25#100",
    );
}

#[test]
#[should_panic]
fn power_of_2_prec_fail() {
    Float::power_of_2_prec(0, 0);
}

#[test]
fn power_of_2_prec_properties() {
    signed_unsigned_pair_gen_var_19::<i64, u64>().test_properties(|(i, prec)| {
        let p = Float::power_of_2_prec(i, prec);
        assert!(p.is_valid());
        assert!(p.is_finite());
        assert!(p.is_power_of_2());
        assert!(p > 0u32);
        assert_eq!(p.get_prec(), Some(prec));
        assert_eq!(Float::power_of_2(i), p);
    });
}

#[test]
fn test_power_of_2() {
    let test = |i: i64, out: &str, out_hex: &str| {
        let p = Float::power_of_2(i);
        assert!(p.is_valid());

        assert_eq!(p.to_string(), out);
        assert_eq!(to_hex_string(&p), out_hex);
    };
    test(0, "1.0", "0x1.0#1");
    test(1, "2.0", "0x2.0#1");
    test(2, "4.0", "0x4.0#1");
    test(3, "8.0", "0x8.0#1");
    test(100, "1.0e30", "0x1.0E+25#1");
    test(-1, "0.5", "0x0.8#1");
    test(-2, "0.2", "0x0.4#1");
    test(-3, "0.1", "0x0.2#1");
    test(-100, "8.0e-31", "0x1.0E-25#1");

    let test = |u: u64, out: &str, out_hex: &str| {
        let p = Float::power_of_2(u);
        assert!(p.is_valid());

        assert_eq!(p.to_string(), out);
        assert_eq!(to_hex_string(&p), out_hex);
    };
    test(0, "1.0", "0x1.0#1");
    test(1, "2.0", "0x2.0#1");
    test(2, "4.0", "0x4.0#1");
    test(3, "8.0", "0x8.0#1");
    test(100, "1.0e30", "0x1.0E+25#1");
}

#[test]
fn power_of_2_properties() {
    signed_gen_var_5::<i64>().test_properties(|i| {
        let p = Float::power_of_2(i);
        assert!(p.is_valid());
        assert!(p.is_finite());
        assert!(p.is_power_of_2());
        assert!(p > 0u32);
        assert_eq!(p.get_prec(), Some(1));
        assert_eq!(
            ComparableFloatRef(&Float::power_of_2_prec(i, 1)),
            ComparableFloatRef(&p)
        );
        if i >= 0 {
            assert_eq!(
                ComparableFloatRef(&Float::power_of_2(i.unsigned_abs())),
                ComparableFloatRef(&p)
            );
        }
    });

    unsigned_gen_var_5::<u64>().test_properties(|u| {
        let p = Float::power_of_2(u);
        assert!(p.is_valid());
        assert!(p.is_finite());
        assert!(p.is_power_of_2());
        assert!(p >= 1u32);
        assert_eq!(p.get_prec(), Some(1));
    });
}
