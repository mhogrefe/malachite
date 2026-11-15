// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_float::big_exponent_float::ComparableBigExponentFloatRef;
use malachite_float::test_util::big_exponent_float::{
    big_exponent_float_exponent_is_small, float_to_big_exponent_float, parse_big_exponent_float_hex,
};
use malachite_float::test_util::generators::*;
use malachite_float::{BigExponentFloat, Float};
use malachite_q::Rational;
use std::cmp::Ordering::{self, *};

fn same(x: &BigExponentFloat, y: &BigExponentFloat) -> bool {
    ComparableBigExponentFloatRef(x) == ComparableBigExponentFloatRef(y)
}

fn same_pair(x: &(BigExponentFloat, Ordering), y: &(BigExponentFloat, Ordering)) -> bool {
    same(&x.0, &y.0) && x.1 == y.1
}

fn float_result_comparable(x: &Float, o: Ordering) -> bool {
    x.get_exponent().map_or_else(
        || !x.is_infinite() && (!x.is_zero() || o == Equal),
        |e| e > Float::MIN_EXPONENT + 1 && e < Float::MAX_EXPONENT - 1,
    )
}

fn positive_difference_prec_round_all(
    x: &BigExponentFloat,
    y: &BigExponentFloat,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = x
        .clone()
        .positive_difference_prec_round(y.clone(), prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(
        &x.clone()
            .positive_difference_prec_round_val_ref(y, prec, rm),
        &r
    ));
    assert!(same_pair(
        &x.positive_difference_prec_round_ref_val(y.clone(), prec, rm),
        &r
    ));
    assert!(same_pair(
        &x.positive_difference_prec_round_ref_ref(y, prec, rm),
        &r
    ));
    let mut z = x.clone();
    let o = z.positive_difference_prec_round_assign(y.clone(), prec, rm);
    assert!(same_pair(&(z, o), &r));
    let mut z = x.clone();
    let o = z.positive_difference_prec_round_assign_ref(y, prec, rm);
    assert!(same_pair(&(z, o), &r));
    if rm == Nearest {
        assert!(same_pair(
            &x.clone().positive_difference_prec(y.clone(), prec),
            &r
        ));
        assert!(same_pair(
            &x.clone().positive_difference_prec_val_ref(y, prec),
            &r
        ));
        assert!(same_pair(
            &x.positive_difference_prec_ref_val(y.clone(), prec),
            &r
        ));
        assert!(same_pair(&x.positive_difference_prec_ref_ref(y, prec), &r));
        let mut z = x.clone();
        let o = z.positive_difference_prec_assign(y.clone(), prec);
        assert!(same_pair(&(z, o), &r));
        let mut z = x.clone();
        let o = z.positive_difference_prec_assign_ref(y, prec);
        assert!(same_pair(&(z, o), &r));
    }
    r
}

fn positive_difference_exact_all(x: &BigExponentFloat, y: &BigExponentFloat) -> BigExponentFloat {
    let r = x.clone().positive_difference(y.clone());
    assert!(r.is_valid());
    assert!(same(&x.clone().positive_difference_val_ref(y), &r));
    assert!(same(&x.positive_difference_ref_val(y.clone()), &r));
    assert!(same(&x.positive_difference_ref_ref(y), &r));
    let mut z = x.clone();
    z.positive_difference_assign(y.clone());
    assert!(same(&z, &r));
    let mut z = x.clone();
    z.positive_difference_assign_ref(y);
    assert!(same(&z, &r));
    r
}

#[test]
fn test_positive_difference() {
    let test = |x: &str, x_hex: &str, y: &str, y_hex: &str, out: &str, out_hex: &str| {
        assert_eq!(parse_big_exponent_float_hex(x_hex).to_string(), x);
        let x = &parse_big_exponent_float_hex(x_hex);
        assert_eq!(parse_big_exponent_float_hex(y_hex).to_string(), y);
        let y = &parse_big_exponent_float_hex(y_hex);
        assert_eq!(parse_big_exponent_float_hex(out_hex).to_string(), out);
        let out = &parse_big_exponent_float_hex(out_hex);
        assert!(same(&positive_difference_exact_all(x, y), out));
    };
    test("NaN", "NaN", "1.0", "0x1.0", "NaN", "NaN");
    test("1.0", "0x1.0", "NaN", "NaN", "NaN", "NaN");
    test(
        "Infinity", "Infinity", "Infinity", "Infinity", "0.0", "0x0.0",
    );
    test(
        "-Infinity",
        "-Infinity",
        "-Infinity",
        "-Infinity",
        "0.0",
        "0x0.0",
    );
    test(
        "Infinity", "Infinity", "1.0", "0x1.0", "Infinity", "Infinity",
    );
    test(
        "1.0",
        "0x1.0",
        "-Infinity",
        "-Infinity",
        "Infinity",
        "Infinity",
    );
    test("1.0", "0x1.0", "Infinity", "Infinity", "0.0", "0x0.0");
    test("1.0", "0x1.0", "2.0", "0x2.0", "0.0", "0x0.0");
    // 2.25 - 1.5 = 0.75
    test("2.25", "0x2.4", "1.5", "0x1.8", "0.75", "0x0.c");

    // 2^N - 2^-N, beyond `Float`'s range, is just below 2^N.
    let test_prec_round = |x: &str,
                           x_hex: &str,
                           y: &str,
                           y_hex: &str,
                           prec: u64,
                           rm: RoundingMode,
                           out: &str,
                           out_hex: &str,
                           o: Ordering| {
        assert_eq!(parse_big_exponent_float_hex(x_hex).to_string(), x);
        let x = &parse_big_exponent_float_hex(x_hex);
        assert_eq!(parse_big_exponent_float_hex(y_hex).to_string(), y);
        let y = &parse_big_exponent_float_hex(y_hex);
        assert_eq!(parse_big_exponent_float_hex(out_hex).to_string(), out);
        let out = &parse_big_exponent_float_hex(out_hex);
        assert!(same_pair(
            &positive_difference_prec_round_all(x, y, prec, rm),
            &(out.clone(), o)
        ));
    };
    test_prec_round(
        "2.4e30102999566398119521",
        "0x1.0E+25000000000000000000",
        "4.2e-30102999566398119522",
        "0x1.0E-25000000000000000000",
        1,
        Floor,
        "1.2e30102999566398119521",
        "0x8.0E+24999999999999999999",
        Less,
    );
    test_prec_round(
        "4.2e-30102999566398119522",
        "0x1.0E-25000000000000000000",
        "2.4e30102999566398119521",
        "0x1.0E+25000000000000000000",
        1,
        Floor,
        "0.0",
        "0x0.0",
        Equal,
    );
}

#[test]
#[should_panic]
fn positive_difference_prec_round_fail() {
    BigExponentFloat::ONE.positive_difference_prec_round(BigExponentFloat::ONE, 0, Floor);
}

#[test]
fn positive_difference_prec_round_properties() {
    big_exponent_float_big_exponent_float_unsigned_rounding_mode_quadruple_gen_var_8()
        .test_properties(|(x, y, prec, rm)| {
            let r = positive_difference_prec_round_all(&x, &y, prec, rm);
            match x.partial_cmp(&y) {
                None => assert!(r.0.is_nan()),
                Some(Greater) => {
                    assert!(same_pair(&x.sub_prec_round_ref_ref(&y, prec, rm), &r));
                }
                _ => assert!(same_pair(&r, &(BigExponentFloat::ZERO, Equal))),
            }
        });
}

#[test]
fn positive_difference_properties() {
    big_exponent_float_pair_gen_var_2().test_properties(|(x, y)| {
        let r = positive_difference_exact_all(&x, &y);
        if !r.is_nan() {
            assert!(r >= 0u32);
            if x > y {
                assert!(same(&r, &(&x - &y)));
            }
        }
    });
}

#[test]
fn positive_difference_prec_round_float_agreement_properties() {
    big_exponent_float_big_exponent_float_unsigned_rounding_mode_quadruple_gen_var_8_float()
        .test_properties(|((fx, fy, prec, rm), (x, y, _, _))| {
            let (f, fo) = fx.positive_difference_prec_round_ref_ref(&fy, prec, rm);
            if float_result_comparable(&f, fo) {
                let r = positive_difference_prec_round_all(&x, &y, prec, rm);
                assert!(same(&r.0, &float_to_big_exponent_float(&f)));
                assert_eq!(r.1, fo);
            }
        });
}

fn positive_difference_rational_all(
    x: &BigExponentFloat,
    y: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = x
        .clone()
        .positive_difference_rational_prec_round(y.clone(), prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(
        &x.clone()
            .positive_difference_rational_prec_round_val_ref(y, prec, rm),
        &r
    ));
    assert!(same_pair(
        &x.positive_difference_rational_prec_round_ref_val(y.clone(), prec, rm),
        &r
    ));
    assert!(same_pair(
        &x.positive_difference_rational_prec_round_ref_ref(y, prec, rm),
        &r
    ));
    let mut z = x.clone();
    let o = z.positive_difference_rational_prec_round_assign(y.clone(), prec, rm);
    assert!(same_pair(&(z, o), &r));
    let mut z = x.clone();
    let o = z.positive_difference_rational_prec_round_assign_ref(y, prec, rm);
    assert!(same_pair(&(z, o), &r));
    if rm == Nearest {
        assert!(same_pair(
            &x.clone().positive_difference_rational_prec(y.clone(), prec),
            &r
        ));
        assert!(same_pair(
            &x.clone().positive_difference_rational_prec_val_ref(y, prec),
            &r
        ));
        assert!(same_pair(
            &x.positive_difference_rational_prec_ref_val(y.clone(), prec),
            &r
        ));
        assert!(same_pair(
            &x.positive_difference_rational_prec_ref_ref(y, prec),
            &r
        ));
        let mut z = x.clone();
        let o = z.positive_difference_rational_prec_assign(y.clone(), prec);
        assert!(same_pair(&(z, o), &r));
        let mut z = x.clone();
        let o = z.positive_difference_rational_prec_assign_ref(y, prec);
        assert!(same_pair(&(z, o), &r));
    }
    r
}

fn rational_positive_difference_all(
    x: &Rational,
    y: &BigExponentFloat,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = BigExponentFloat::rational_positive_difference_big_exponent_float_prec_round(
        x.clone(),
        y.clone(),
        prec,
        rm,
    );
    assert!(r.0.is_valid());
    assert!(same_pair(
        &BigExponentFloat::rational_positive_difference_big_exponent_float_prec_round_val_ref(
            x.clone(),
            y,
            prec,
            rm
        ),
        &r
    ));
    assert!(same_pair(
        &BigExponentFloat::rational_positive_difference_big_exponent_float_prec_round_ref_val(
            x,
            y.clone(),
            prec,
            rm
        ),
        &r
    ));
    assert!(same_pair(
        &BigExponentFloat::rational_positive_difference_big_exponent_float_prec_round_ref_ref(
            x, y, prec, rm
        ),
        &r
    ));
    if rm == Nearest {
        assert!(same_pair(
            &BigExponentFloat::rational_positive_difference_big_exponent_float_prec(
                x.clone(),
                y.clone(),
                prec
            ),
            &r
        ));
        assert!(same_pair(
            &BigExponentFloat::rational_positive_difference_big_exponent_float_prec_val_ref(
                x.clone(),
                y,
                prec
            ),
            &r
        ));
        assert!(same_pair(
            &BigExponentFloat::rational_positive_difference_big_exponent_float_prec_ref_val(
                x,
                y.clone(),
                prec
            ),
            &r
        ));
        assert!(same_pair(
            &BigExponentFloat::rational_positive_difference_big_exponent_float_prec_ref_ref(
                x, y, prec
            ),
            &r
        ));
    }
    r
}

#[test]
fn test_positive_difference_rational() {
    let test = |x: &str,
                x_hex: &str,
                y: &str,
                prec: u64,
                rm: RoundingMode,
                out: &str,
                out_hex: &str,
                o: Ordering| {
        assert_eq!(parse_big_exponent_float_hex(x_hex).to_string(), x);
        let x = &parse_big_exponent_float_hex(x_hex);
        assert_eq!(parse_big_exponent_float_hex(out_hex).to_string(), out);
        let out = &parse_big_exponent_float_hex(out_hex);
        let y = Rational::from_str(y).unwrap();
        let r = positive_difference_rational_all(x, &y, prec, rm);
        assert!(same(&r.0, out));
        assert_eq!(r.1, o);
    };
    test("NaN", "NaN", "1/3", 10, Nearest, "NaN", "NaN", Equal);
    test(
        "Infinity", "Infinity", "1/3", 10, Nearest, "Infinity", "Infinity", Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "1/3",
        10,
        Nearest,
        "0.0",
        "0x0.0",
        Equal,
    );
    test("0.25", "0x0.4", "1/3", 10, Nearest, "0.0", "0x0.0", Equal);
    test("0.0", "0x0.0", "1/3", 10, Nearest, "0.0", "0x0.0", Equal);
    // 1/2 - 1/3 = 1/6
    let o =
        BigExponentFloat::from_rational_prec_round(Rational::from_unsigneds(1u8, 6), 10, Floor).1;
    test("0.50", "0x0.8", "1/3", 10, Floor, "0.1665", "0x0.2aa", o);
    // 2^N - 1/3 rounds to 2^N at any moderate precision.
    test(
        "2.4e30102999566398119521",
        "0x1.0E+25000000000000000000",
        "1/3",
        10,
        Nearest,
        "2.4e30102999566398119521",
        "0x1.0E+25000000000000000000",
        Greater,
    );
}

#[test]
fn test_rational_positive_difference() {
    let test = |x: &str,
                y: &str,
                y_hex: &str,
                prec: u64,
                rm: RoundingMode,
                out: &str,
                out_hex: &str,
                o: Ordering| {
        assert_eq!(parse_big_exponent_float_hex(y_hex).to_string(), y);
        let y = &parse_big_exponent_float_hex(y_hex);
        assert_eq!(parse_big_exponent_float_hex(out_hex).to_string(), out);
        let out = &parse_big_exponent_float_hex(out_hex);
        let x = Rational::from_str(x).unwrap();
        let r = rational_positive_difference_all(&x, y, prec, rm);
        assert!(same(&r.0, out));
        assert_eq!(r.1, o);
    };
    test("1/3", "NaN", "NaN", 10, Nearest, "NaN", "NaN", Equal);
    test(
        "1/3",
        "-Infinity",
        "-Infinity",
        10,
        Nearest,
        "Infinity",
        "Infinity",
        Equal,
    );
    test(
        "1/3", "Infinity", "Infinity", 10, Nearest, "0.0", "0x0.0", Equal,
    );
    test("1/3", "0.50", "0x0.8", 10, Nearest, "0.0", "0x0.0", Equal);
    test(
        "1/3",
        "2.4e30102999566398119521",
        "0x1.0E+25000000000000000000",
        10,
        Nearest,
        "0.0",
        "0x0.0",
        Equal,
    );
    // 1/3 - 1/4 = 1/12, rounded in each direction.
    let o =
        BigExponentFloat::from_rational_prec_round(Rational::from_unsigneds(1u8, 12), 10, Floor).1;
    test("1/3", "0.25", "0x0.4", 10, Floor, "0.08325", "0x0.155", o);
    let o =
        BigExponentFloat::from_rational_prec_round(Rational::from_unsigneds(1u8, 12), 10, Ceiling)
            .1;
    test(
        "1/3", "0.25", "0x0.4", 10, Ceiling, "0.083374", "0x0.1558", o,
    );
    let o =
        BigExponentFloat::from_rational_prec_round(Rational::from_unsigneds(1u8, 12), 10, Down).1;
    test("1/3", "0.25", "0x0.4", 10, Down, "0.08325", "0x0.155", o);
    let o = BigExponentFloat::from_rational_prec_round(Rational::from_unsigneds(1u8, 12), 10, Up).1;
    test("1/3", "0.25", "0x0.4", 10, Up, "0.083374", "0x0.1558", o);
    let o =
        BigExponentFloat::from_rational_prec_round(Rational::from_unsigneds(1u8, 12), 10, Nearest)
            .1;
    test(
        "1/3", "0.25", "0x0.4", 10, Nearest, "0.083374", "0x0.1558", o,
    );
    // 1/3 + 2^N rounds to 2^N at any moderate precision.
    test(
        "1/3",
        "-2.4e30102999566398119521",
        "-0x1.0E+25000000000000000000",
        10,
        Nearest,
        "2.4e30102999566398119521",
        "0x1.0E+25000000000000000000",
        Less,
    );
}

#[test]
fn positive_difference_rational_prec_round_properties() {
    big_exponent_float_rational_unsigned_rounding_mode_quadruple_gen_var_7().test_properties(
        |(x, y, prec, rm)| {
            let r = positive_difference_rational_all(&x, &y, prec, rm);
            if x.is_finite() && big_exponent_float_exponent_is_small(&x) {
                let d = Rational::exact_from(&x) - &y;
                let exact = if d > 0u32 { d } else { Rational::ZERO };
                assert!(same_pair(
                    &r,
                    &BigExponentFloat::from_rational_prec_round(exact, prec, rm)
                ));
            }
        },
    );
    big_exponent_float_rational_unsigned_rounding_mode_quadruple_gen_var_8().test_properties(
        |(y, x, prec, rm)| {
            let r = rational_positive_difference_all(&x, &y, prec, rm);
            if y.is_finite() && big_exponent_float_exponent_is_small(&y) {
                let d = &x - Rational::exact_from(&y);
                let exact = if d > 0u32 { d } else { Rational::ZERO };
                assert!(same_pair(
                    &r,
                    &BigExponentFloat::from_rational_prec_round(exact, prec, rm)
                ));
            }
        },
    );
}

#[test]
fn positive_difference_rational_float_agreement_properties() {
    big_exponent_float_rational_unsigned_rounding_mode_quadruple_gen_var_7_float().test_properties(
        |((fx, y, prec, rm), (x, _, _, _))| {
            let (f, fo) = fx.positive_difference_rational_prec_round_ref_ref(&y, prec, rm);
            if float_result_comparable(&f, fo) {
                let r = positive_difference_rational_all(&x, &y, prec, rm);
                assert!(same(&r.0, &float_to_big_exponent_float(&f)));
                assert_eq!(r.1, fo);
            }
        },
    );
    big_exponent_float_rational_unsigned_rounding_mode_quadruple_gen_var_8_float().test_properties(
        |((fy, x, prec, rm), (y, _, _, _))| {
            let (f, fo) =
                Float::rational_positive_difference_float_prec_round_ref_ref(&x, &fy, prec, rm);
            if float_result_comparable(&f, fo) {
                let r = rational_positive_difference_all(&x, &y, prec, rm);
                assert!(same(&r.0, &float_to_big_exponent_float(&f)));
                assert_eq!(r.1, fo);
            }
        },
    );
}
