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
fn test_catalans_constant_prec_round() {
    let cases: &[(u64, RoundingMode, &str, Ordering)] = &[
        (1, Floor, "0x0.8", Less),
        (1, Ceiling, "0x1.0", Greater),
        (1, Down, "0x0.8", Less),
        (1, Up, "0x1.0", Greater),
        (1, Nearest, "0x1.0", Greater),
        (2, Floor, "0x0.c", Less),
        (2, Ceiling, "0x1.0", Greater),
        (2, Down, "0x0.c", Less),
        (2, Up, "0x1.0", Greater),
        (2, Nearest, "0x1.0", Greater),
        (10, Floor, "0x0.ea4", Less),
        (10, Ceiling, "0x0.ea8", Greater),
        (10, Down, "0x0.ea4", Less),
        (10, Up, "0x0.ea8", Greater),
        (10, Nearest, "0x0.ea8", Greater),
        (100, Floor, "0x0.ea7cb89f409ae845215822e37", Less),
        (100, Ceiling, "0x0.ea7cb89f409ae845215822e38", Greater),
        (100, Down, "0x0.ea7cb89f409ae845215822e37", Less),
        (100, Up, "0x0.ea7cb89f409ae845215822e38", Greater),
        (100, Nearest, "0x0.ea7cb89f409ae845215822e38", Greater),
    ];
    for &(p, rm, out, o) in cases {
        let (x, o_alt) = BigExponentFloat::catalans_constant_prec_round(p, rm);
        assert!(x.is_valid());
        assert_eq!(format!("{x:?}"), out, "{:?}", (p, rm));
        assert_eq!(o_alt, o);
    }
}

#[test]
#[should_panic]
fn catalans_constant_prec_round_fail_1() {
    BigExponentFloat::catalans_constant_prec_round(0, Floor);
}

#[test]
#[should_panic]
fn catalans_constant_prec_round_fail_2() {
    BigExponentFloat::catalans_constant_prec_round(10, Exact);
}

#[test]
fn catalans_constant_prec_round_properties() {
    unsigned_rounding_mode_pair_gen_var_4().test_properties(|(p, rm)| {
        let (x, o) = BigExponentFloat::catalans_constant_prec_round(p, rm);
        assert!(x.is_valid());
        let (y, o_alt) = Float::catalans_constant_prec_round(p, rm);
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
            let (x_alt, o_alt) = BigExponentFloat::catalans_constant_prec(p);
            assert!(same(&x_alt, &x));
            assert_eq!(o_alt, o);
        }
    });
}

#[test]
fn test_catalans_constant_prec() {
    let cases: &[(u64, &str, Ordering)] = &[
        (1, "0x1.0", Greater),
        (2, "0x1.0", Greater),
        (10, "0x0.ea8", Greater),
        (100, "0x0.ea7cb89f409ae845215822e38", Greater),
    ];
    for &(p, out, o) in cases {
        let (x, o_alt) = BigExponentFloat::catalans_constant_prec(p);
        assert!(x.is_valid());
        assert_eq!(format!("{x:?}"), out, "{:?}", (p));
        assert_eq!(o_alt, o);
    }
}

#[test]
#[should_panic]
fn catalans_constant_prec_fail_1() {
    BigExponentFloat::catalans_constant_prec(0);
}

#[test]
fn catalans_constant_prec_properties() {
    unsigned_gen_var_11().test_properties(|p| {
        let (x, o) = BigExponentFloat::catalans_constant_prec(p);
        assert!(x.is_valid());
        let (y, o_alt) = Float::catalans_constant_prec(p);
        assert!(same(&x, &BigExponentFloat::from(y)));
        assert_eq!(o_alt, o);
        assert_ne!(o, Equal);
        assert!(x > 0u32);
    });
}
