// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::test_util::generators::{
    unsigned_gen_var_11, unsigned_rounding_mode_pair_gen_var_4,
};
use malachite_float::big_exponent_float::ComparableBigExponentFloatRef;
use malachite_float::{BigExponentFloat, Float};
use std::cmp::Ordering::{self, *};

fn same(x: &BigExponentFloat, y: &BigExponentFloat) -> bool {
    ComparableBigExponentFloatRef(x) == ComparableBigExponentFloatRef(y)
}

#[test]
fn test_log_2_10_prec_round() {
    let test = |p: u64, rm: RoundingMode, out: &str, out_hex: &str, o: Ordering| {
        let (x, o_alt) = BigExponentFloat::log_2_10_prec_round(p, rm);
        assert!(x.is_valid());
        assert_eq!(format!("{x:?}"), out_hex, "{:?}", (p, rm));
        assert_eq!(x.to_string(), out);
        assert_eq!(o_alt, o);
    };
    test(1, Floor, "2.0", "0x2.0", Less);
    test(1, Ceiling, "4.0", "0x4.0", Greater);
    test(1, Down, "2.0", "0x2.0", Less);
    test(1, Up, "4.0", "0x4.0", Greater);
    test(1, Nearest, "4.0", "0x4.0", Greater);
    test(2, Floor, "3.0", "0x3.0", Less);
    test(2, Ceiling, "4.0", "0x4.0", Greater);
    test(2, Down, "3.0", "0x3.0", Less);
    test(2, Up, "4.0", "0x4.0", Greater);
    test(2, Nearest, "3.0", "0x3.0", Less);
    test(10, Floor, "3.320", "0x3.52", Less);
    test(10, Ceiling, "3.3242", "0x3.53", Greater);
    test(10, Down, "3.320", "0x3.52", Less);
    test(10, Up, "3.3242", "0x3.53", Greater);
    test(10, Nearest, "3.320", "0x3.52", Less);
    test(
        100,
        Floor,
        "3.321928094887362347870319429487",
        "0x3.5269e12f346e2bf924afdbfd",
        Less,
    );
    test(
        100,
        Ceiling,
        "3.3219280948873623478703194294898",
        "0x3.5269e12f346e2bf924afdbfd4",
        Greater,
    );
    test(
        100,
        Down,
        "3.321928094887362347870319429487",
        "0x3.5269e12f346e2bf924afdbfd",
        Less,
    );
    test(
        100,
        Up,
        "3.3219280948873623478703194294898",
        "0x3.5269e12f346e2bf924afdbfd4",
        Greater,
    );
    test(
        100,
        Nearest,
        "3.3219280948873623478703194294898",
        "0x3.5269e12f346e2bf924afdbfd4",
        Greater,
    );
}

#[test]
#[should_panic]
fn log_2_10_prec_round_fail_1() {
    BigExponentFloat::log_2_10_prec_round(0, Floor);
}

#[test]
#[should_panic]
fn log_2_10_prec_round_fail_2() {
    BigExponentFloat::log_2_10_prec_round(10, Exact);
}

#[test]
fn log_2_10_prec_round_properties() {
    unsigned_rounding_mode_pair_gen_var_4().test_properties(|(p, rm)| {
        let (x, o) = BigExponentFloat::log_2_10_prec_round(p, rm);
        assert!(x.is_valid());
        let (y, o_alt) = Float::log_2_10_prec_round(p, rm);
        assert!(same(&x, &BigExponentFloat::from(y)));
        assert_eq!(o_alt, o);
        assert_ne!(o, Equal);
        assert!(x > 0u32);
        match rm {
            Floor | Down => assert_eq!(o, Less),
            Ceiling | Up => assert_eq!(o, Greater),
            _ => {}
        }
        if rm == Nearest {
            let (x_alt, o_alt) = BigExponentFloat::log_2_10_prec(p);
            assert!(same(&x_alt, &x));
            assert_eq!(o_alt, o);
        }
    });
}

#[test]
fn test_log_2_10_prec() {
    let test = |p: u64, out: &str, out_hex: &str, o: Ordering| {
        let (x, o_alt) = BigExponentFloat::log_2_10_prec(p);
        assert!(x.is_valid());
        assert_eq!(format!("{x:?}"), out_hex, "{:?}", (p));
        assert_eq!(x.to_string(), out);
        assert_eq!(o_alt, o);
    };
    test(1, "4.0", "0x4.0", Greater);
    test(2, "3.0", "0x3.0", Less);
    test(10, "3.320", "0x3.52", Less);
    test(
        100,
        "3.3219280948873623478703194294898",
        "0x3.5269e12f346e2bf924afdbfd4",
        Greater,
    );
}

#[test]
#[should_panic]
fn log_2_10_prec_fail_1() {
    BigExponentFloat::log_2_10_prec(0);
}

#[test]
fn log_2_10_prec_properties() {
    unsigned_gen_var_11().test_properties(|p| {
        let (x, o) = BigExponentFloat::log_2_10_prec(p);
        assert!(x.is_valid());
        let (y, o_alt) = Float::log_2_10_prec(p);
        assert!(same(&x, &BigExponentFloat::from(y)));
        assert_eq!(o_alt, o);
        assert_ne!(o, Equal);
        assert!(x > 0u32);
    });
}
