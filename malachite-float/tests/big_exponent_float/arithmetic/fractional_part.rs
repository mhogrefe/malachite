// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::traits::One;
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_float::BigExponentFloat;
use malachite_float::big_exponent_float::ComparableBigExponentFloatRef;
use malachite_float::test_util::big_exponent_float::{
    big_exponent_float_exponent_is_small, float_to_big_exponent_float, parse_big_exponent_float_hex,
};
use malachite_float::test_util::generators::*;
use malachite_q::Rational;
use std::cmp::Ordering::{self, *};

fn same(x: &BigExponentFloat, y: &BigExponentFloat) -> bool {
    ComparableBigExponentFloatRef(x) == ComparableBigExponentFloatRef(y)
}

fn same_pair(x: &(BigExponentFloat, Ordering), y: &(BigExponentFloat, Ordering)) -> bool {
    same(&x.0, &y.0) && x.1 == y.1
}

fn fractional_part_prec_round_all(
    x: &BigExponentFloat,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = x.clone().fractional_part_prec_round(prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(&x.fractional_part_prec_round_ref(prec, rm), &r));
    if rm == Nearest {
        assert!(same_pair(&x.clone().fractional_part_prec(prec), &r));
        assert!(same_pair(&x.fractional_part_prec_ref(prec), &r));
    }
    r
}

fn fractional_part_all(x: &BigExponentFloat) -> BigExponentFloat {
    let r = x.clone().fractional_part();
    assert!(r.is_valid());
    assert!(same(&x.fractional_part_ref(), &r));
    r
}

fn integer_and_fractional_parts_all(x: &BigExponentFloat) -> (BigExponentFloat, BigExponentFloat) {
    let r = x.clone().integer_and_fractional_parts();
    let s = x.integer_and_fractional_parts_ref();
    assert!(same(&r.0, &s.0) && same(&r.1, &s.1));
    r
}

#[test]
fn test_fractional_part() {
    let test = |x: &str, x_hex: &str, out: &str, out_hex: &str| {
        assert_eq!(parse_big_exponent_float_hex(x_hex).to_string(), x);
        let x = &parse_big_exponent_float_hex(x_hex);
        assert_eq!(parse_big_exponent_float_hex(out_hex).to_string(), out);
        let out = &parse_big_exponent_float_hex(out_hex);
        assert!(same(&fractional_part_all(x), out));
    };
    test("NaN", "NaN", "NaN", "NaN");
    test("-Infinity", "-Infinity", "0.0", "0x0.0");
    test("3.0", "0x3.0", "0.0", "0x0.0");
    // -3.25 = -3 - 0.25
    test("-3.25", "-0x3.4", "-0.25", "-0x0.4");

    let test_parts =
        |x: &str, x_hex: &str, i_out: &str, i_out_hex: &str, f_out: &str, f_out_hex: &str| {
            assert_eq!(parse_big_exponent_float_hex(x_hex).to_string(), x);
            let x = &parse_big_exponent_float_hex(x_hex);
            assert_eq!(parse_big_exponent_float_hex(i_out_hex).to_string(), i_out);
            let i_out = &parse_big_exponent_float_hex(i_out_hex);
            assert_eq!(parse_big_exponent_float_hex(f_out_hex).to_string(), f_out);
            let f_out = &parse_big_exponent_float_hex(f_out_hex);
            let (i, f) = integer_and_fractional_parts_all(x);
            assert!(same(&i, i_out));
            assert!(same(&f, f_out));
        };
    test_parts("-3.25", "-0x3.4", "-3.0", "-0x3.0", "-0.25", "-0x0.4");
    test_parts(
        "Infinity", "Infinity", "Infinity", "Infinity", "0.0", "0x0.0",
    );
    // Exponents beyond `Float`'s range: 3 * 2^N is an integer, and 3 * 2^-N is a fraction.
    test_parts(
        "7.1e30102999566398119521",
        "0x3.0E+25000000000000000000",
        "7.1e30102999566398119521",
        "0x3.0E+25000000000000000000",
        "0.0",
        "0x0.0",
    );
    test_parts(
        "-1.3e-30102999566398119521",
        "-0x3.0E-25000000000000000000",
        "0.0",
        "0x0.0",
        "-1.3e-30102999566398119521",
        "-0x3.0E-25000000000000000000",
    );

    // -3.375 = -3 - 0.375, and -0.375 rounds down to -0.5 at 1 bit.
    let test_prec_round = |x: &str,
                           x_hex: &str,
                           prec: u64,
                           rm: RoundingMode,
                           out: &str,
                           out_hex: &str,
                           o: Ordering| {
        assert_eq!(parse_big_exponent_float_hex(x_hex).to_string(), x);
        let x = &parse_big_exponent_float_hex(x_hex);
        assert_eq!(parse_big_exponent_float_hex(out_hex).to_string(), out);
        let out = &parse_big_exponent_float_hex(out_hex);
        assert!(same_pair(
            &fractional_part_prec_round_all(x, prec, rm),
            &(out.clone(), o)
        ));
    };
    test_prec_round("-3.38", "-0x3.6", 1, Floor, "-0.50", "-0x0.8", Less);
}

#[test]
#[should_panic]
fn fractional_part_prec_round_fail() {
    BigExponentFloat::ONE.fractional_part_prec_round(0, Floor);
}

#[test]
fn fractional_part_prec_round_properties() {
    big_exponent_float_unsigned_rounding_mode_triple_gen_var_8().test_properties(
        |(x, prec, rm)| {
            let r = fractional_part_prec_round_all(&x, prec, rm);
            let exact = fractional_part_all(&x);
            assert!(same_pair(&exact.round_to_prec_round_ref(prec, rm), &r));
            // `Exact` is only known to be valid for the fractional part.
            if rm != Exact {
                let (ip, fp) = x.integer_and_fractional_parts_prec_round_ref(prec, prec, rm);
                assert!(same_pair(&fp, &r));
                assert!(same_pair(
                    &ip,
                    &x.round_to_integer_then_prec_round_ref(Down, prec, rm)
                ));
            }
            if rm == Nearest {
                let (ip2, fp2) = x.integer_and_fractional_parts_prec_ref(prec, prec);
                let (ip, fp) = x.integer_and_fractional_parts_prec_round_ref(prec, prec, rm);
                assert!(same_pair(&ip2, &ip) && same_pair(&fp2, &fp));
            }
        },
    );
}

#[test]
fn fractional_part_properties() {
    big_exponent_float_gen().test_properties(|x| {
        let f = fractional_part_all(&x);
        let (i, f2) = integer_and_fractional_parts_all(&x);
        assert!(same(&f, &f2));
        if x.is_finite() {
            // The fractional part is less than 1 in magnitude, has x's sign, and the parts add up
            // to x.
            assert!(f.lt_abs(&BigExponentFloat::ONE));
            assert!(f == 0u32 || f.is_sign_positive() == x.is_sign_positive());
            if big_exponent_float_exponent_is_small(&x) {
                assert_eq!(
                    Rational::exact_from(&i) + Rational::exact_from(&f),
                    Rational::exact_from(&x)
                );
            }
        }
    });
}

#[test]
fn fractional_part_float_agreement_properties() {
    big_exponent_float_unsigned_rounding_mode_triple_gen_var_8_float().test_properties(
        |((fx, prec, rm), (x, _, _))| {
            let (f, fo) = fx.fractional_part_prec_round_ref(prec, rm);
            let r = fractional_part_prec_round_all(&x, prec, rm);
            assert!(same_pair(&r, &(float_to_big_exponent_float(&f), fo)));
            let ((fi, fio), (ff, ffo)) =
                fx.integer_and_fractional_parts_prec_round_ref(prec, prec, rm);
            let (ip, fp) = x.integer_and_fractional_parts_prec_round_ref(prec, prec, rm);
            if fi.is_finite() || !x.is_finite() {
                assert!(same_pair(&ip, &(float_to_big_exponent_float(&fi), fio)));
            }
            assert!(same_pair(&fp, &(float_to_big_exponent_float(&ff), ffo)));
        },
    );
}
