// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::traits::NaN;
use malachite_base::num::conversion::traits::FromStringBase;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_float::BigExponentFloat;
use malachite_float::big_exponent_float::ComparableBigExponentFloatRef;
use malachite_float::test_util::generators::*;
use malachite_q::Rational;
use std::cmp::Ordering::{self, *};
use std::str::FromStr;

fn same(x: &BigExponentFloat, y: &BigExponentFloat) -> bool {
    ComparableBigExponentFloatRef(x) == ComparableBigExponentFloatRef(y)
}

fn hex(s: &str) -> BigExponentFloat {
    BigExponentFloat::from_string_base(16, s).unwrap()
}

fn sinh_cosh_prec_round_all(
    x: &BigExponentFloat,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, BigExponentFloat, Ordering, Ordering) {
    let (s, o_s) = x.sinh_prec_round_ref(prec, rm);
    let (c, o_c) = x.cosh_prec_round_ref(prec, rm);
    for r in [x.clone().sinh_cosh_prec_round(prec, rm), x.sinh_cosh_prec_round_ref(prec, rm)] {
        assert!(same(&r.0, &s) && same(&r.1, &c) && r.2 == o_s && r.3 == o_c);
    }
    let mut z = x.clone();
    let mut w = BigExponentFloat::NAN;
    let os = z.sinh_cosh_prec_round_assign(&mut w, prec, rm);
    assert!(same(&z, &s) && same(&w, &c) && os == (o_s, o_c));
    if rm == Nearest {
        for r in [x.clone().sinh_cosh_prec(prec), x.sinh_cosh_prec_ref(prec)] {
            assert!(same(&r.0, &s) && same(&r.1, &c) && r.2 == o_s && r.3 == o_c);
        }
        let mut z = x.clone();
        let mut w = BigExponentFloat::NAN;
        let os = z.sinh_cosh_prec_assign(&mut w, prec);
        assert!(same(&z, &s) && same(&w, &c) && os == (o_s, o_c));
    }
    (s, c, o_s, o_c)
}

fn sinh_cosh_rational_prec_round_all(
    x: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, BigExponentFloat, Ordering, Ordering) {
    let (s, o_s) = BigExponentFloat::sinh_rational_prec_round_ref(x, prec, rm);
    let (c, o_c) = BigExponentFloat::cosh_rational_prec_round_ref(x, prec, rm);
    for r in [
        BigExponentFloat::sinh_cosh_rational_prec_round(x.clone(), prec, rm),
        BigExponentFloat::sinh_cosh_rational_prec_round_ref(x, prec, rm),
    ] {
        assert!(same(&r.0, &s) && same(&r.1, &c) && r.2 == o_s && r.3 == o_c);
    }
    if rm == Nearest {
        for r in [
            BigExponentFloat::sinh_cosh_rational_prec(x.clone(), prec),
            BigExponentFloat::sinh_cosh_rational_prec_ref(x, prec),
        ] {
            assert!(same(&r.0, &s) && same(&r.1, &c) && r.2 == o_s && r.3 == o_c);
        }
    }
    (s, c, o_s, o_c)
}

#[test]
fn test_sinh_cosh_prec_round() {
    let r = sinh_cosh_prec_round_all(&hex("0x1"), 10, Floor);
    assert_eq!(format!("{:?}", r.0), "0x1.2c8");
    assert_eq!(r.0.to_string(), "1.1738");
    assert_eq!(format!("{:?}", r.1), "0x1.8b");
    assert_eq!(r.1.to_string(), "1.543");
    assert_eq!((r.2, r.3), (Less, Less));
    let r = sinh_cosh_prec_round_all(&hex("0x0"), 10, Exact);
    assert_eq!(format!("{:?}", r.0), "0x0.0");
    assert_eq!(r.0.to_string(), "0.0");
    assert_eq!(format!("{:?}", r.1), "0x1.0");
    assert_eq!(r.1.to_string(), "1.0");
    assert_eq!((r.2, r.3), (Equal, Equal));
    let r = sinh_cosh_prec_round_all(&hex("NaN"), 10, Floor);
    assert_eq!(format!("{:?}", r.0), "NaN");
    assert_eq!(r.0.to_string(), "NaN");
    assert_eq!(format!("{:?}", r.1), "NaN");
    assert_eq!(r.1.to_string(), "NaN");
    assert_eq!((r.2, r.3), (Equal, Equal));
    let r = sinh_cosh_prec_round_all(&hex("-Infinity"), 10, Floor);
    assert_eq!(format!("{:?}", r.0), "-Infinity");
    assert_eq!(r.0.to_string(), "-Infinity");
    assert_eq!(format!("{:?}", r.1), "Infinity");
    assert_eq!(r.1.to_string(), "Infinity");
    assert_eq!((r.2, r.3), (Equal, Equal));
    let r = sinh_cosh_prec_round_all(&hex("0x1.0E+8"), 20, Nearest);
    assert_eq!(format!("{:?}", r.0), "0x3.4b0dcE+1549082004");
    assert_eq!(r.0.to_string(), "2.0262848e1865280596");
    assert_eq!(format!("{:?}", r.1), "0x3.4b0dcE+1549082004");
    assert_eq!(r.1.to_string(), "2.0262848e1865280596");
    assert_eq!((r.2, r.3), (Greater, Greater));
    let r = sinh_cosh_prec_round_all(&hex("0x3.0E-10000000000"), 10, Ceiling);
    assert_eq!(format!("{:?}", r.0), "0x3.01E-10000000000");
    assert_eq!(r.0.to_string(), "8.2878e-12041199827");
    assert_eq!(format!("{:?}", r.1), "0x1.008");
    assert_eq!(r.1.to_string(), "1.0020");
    assert_eq!((r.2, r.3), (Greater, Greater));
    let r = sinh_cosh_rational_prec_round_all(&Rational::from_str("1/3").unwrap(), 20, Nearest);
    assert_eq!(format!("{:?}", r.0), "0x0.56ec2");
    assert_eq!(r.0.to_string(), "0.3395405");
    assert_eq!(format!("{:?}", r.1), "0x1.0e5ac");
    assert_eq!(r.1.to_string(), "1.056072");
    assert_eq!((r.2, r.3), (Less, Greater));
    let r = sinh_cosh_rational_prec_round_all(&Rational::from_str("0").unwrap(), 10, Exact);
    assert_eq!(format!("{:?}", r.0), "0x0.0");
    assert_eq!(r.0.to_string(), "0.0");
    assert_eq!(format!("{:?}", r.1), "0x1.0");
    assert_eq!(r.1.to_string(), "1.0");
    assert_eq!((r.2, r.3), (Equal, Equal));
    let r = sinh_cosh_rational_prec_round_all(
        &Rational::from_str("-1000000000000").unwrap(),
        20,
        Floor,
    );
    assert_eq!(format!("{:?}", r.0), "-0xf.996bE+360673760221");
    assert_eq!(r.0.to_string(), "-8.9289000e434294481902");
    assert_eq!(format!("{:?}", r.1), "0xf.996aE+360673760221");
    assert_eq!(r.1.to_string(), "8.928891e434294481902");
    assert_eq!((r.2, r.3), (Less, Less));
}

#[test]
fn sinh_cosh_prec_round_properties() {
    big_exponent_float_unsigned_rounding_mode_triple_gen_var_9().test_properties(
        |(x, prec, rm)| {
            sinh_cosh_prec_round_all(&x, prec, rm);
        },
    );
    rational_unsigned_rounding_mode_triple_gen_var_19().test_properties(|(x, prec, rm)| {
        sinh_cosh_rational_prec_round_all(&x, prec, rm);
    });
}
