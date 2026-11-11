// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::conversion::traits::ConvertibleFrom;
use malachite_base::rounding_modes::RoundingMode::*;
use malachite_float::big_exponent_float::conversion::from_rational::*;
use malachite_float::test_util::generators::rational_unsigned_rounding_mode_triple_gen_var_1;
use malachite_float::{BigExponentFloat, Float};
use malachite_q::Rational;
use malachite_q::test_util::generators::rational_gen;
use std::cmp::Ordering::*;
use std::str::FromStr;

#[test]
fn test_from_rational_prec_round() {
    let test = |s, prec, rm, out: &str, o| {
        let q = Rational::from_str(s).unwrap();
        let (x, o_out) = BigExponentFloat::from_rational_prec_round_ref(&q, prec, rm);
        assert!(x.is_valid());
        assert_eq!(format!("{x:#x}"), out);
        assert_eq!(o_out, o);
        let (x, o_out) = BigExponentFloat::from_rational_prec_round(q, prec, rm);
        assert_eq!(format!("{x:#x}"), out);
        assert_eq!(o_out, o);
    };
    test("0", 1, Floor, "0x0.0", Equal);
    test("1/3", 10, Floor, "0x0.554", Less);
    test("1/3", 10, Nearest, "0x0.556", Greater);
    test("-1/3", 10, Floor, "-0x0.556", Less);
    test("3/8", 1, Floor, "0x0.4", Less);
    test("3/8", 2, Exact, "0x0.6", Equal);
    test("255/256", 4, Up, "0x1.0", Greater);
}

#[test]
fn test_try_from_rational() {
    let test = |s, out: Result<&str, BigExponentFloatFromRationalError>| {
        let q = Rational::from_str(s).unwrap();
        assert_eq!(
            BigExponentFloat::try_from(&q).map(|x| format!("{x:#x}")),
            out.map(ToString::to_string)
        );
        assert_eq!(BigExponentFloat::convertible_from(&q), out.is_ok());
    };
    test("0", Ok("0x0.0"));
    test("3/8", Ok("0x0.6"));
    test("-3/8", Ok("-0x0.6"));
    test("1/3", Err(BigExponentFloatFromRationalError));
    test("1/1267650600228229401496703205376", Ok("0x1.0E-25"));
}

#[test]
#[should_panic]
fn from_rational_prec_round_fail() {
    BigExponentFloat::from_rational_prec_round(Rational::from_signeds(1, 3), 10, Exact);
}

#[test]
fn from_rational_properties() {
    rational_unsigned_rounding_mode_triple_gen_var_1().test_properties(|(q, prec, rm)| {
        let (x, o) = BigExponentFloat::from_rational_prec_round_ref(&q, prec, rm);
        assert!(x.is_valid());
        let (x_alt, o_alt) = BigExponentFloat::from_rational_prec_round(q.clone(), prec, rm);
        assert_eq!(format!("{x_alt:#x}"), format!("{x:#x}"));
        assert_eq!(o_alt, o);
        if let Some(p) = x.get_prec() {
            assert!(p <= prec);
        }
        // - the ordering is the comparison with the exact value
        assert_eq!(Rational::try_from(&x).unwrap().cmp(&q), o);
        // - agreement with `Float`, which has the same exponent range for these inputs
        let (f, o_f) = Float::from_rational_prec_round_ref(&q, prec, rm);
        assert_eq!(
            format!("{:#x}", BigExponentFloat::from(&f)),
            format!("{x:#x}")
        );
        assert_eq!(o_f, o);
        if rm == Nearest {
            let (x_alt, o_alt) = BigExponentFloat::from_rational_prec_ref(&q, prec);
            assert_eq!(format!("{x_alt:#x}"), format!("{x:#x}"));
            assert_eq!(o_alt, o);
        }
    });

    rational_gen().test_properties(|q| {
        let convertible = BigExponentFloat::convertible_from(&q);
        let result = BigExponentFloat::try_from(&q);
        assert_eq!(result.is_ok(), convertible);
        if let Ok(x) = result {
            assert!(x.is_valid());
            assert_eq!(Rational::try_from(&x).unwrap(), q);
            assert_eq!(x.is_zero(), q == 0u32);
        }
    });
}
