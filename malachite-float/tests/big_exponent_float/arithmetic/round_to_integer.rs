// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::Abs;
use malachite_base::num::basic::traits::{One, Two};
use malachite_base::num::conversion::traits::{ExactFrom, RoundingFrom};
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::rounding_modes::exhaustive::exhaustive_rounding_modes;
use malachite_float::BigExponentFloat;
use malachite_float::big_exponent_float::ComparableBigExponentFloatRef;
use malachite_float::test_util::big_exponent_float::{
    big_exponent_float_exponent_is_small, float_to_big_exponent_float, parse_big_exponent_float_hex,
};
use malachite_float::test_util::generators::*;
use malachite_nz::integer::Integer;
use malachite_q::Rational;
use std::cmp::Ordering::{self, *};

fn same(x: &BigExponentFloat, y: &BigExponentFloat) -> bool {
    ComparableBigExponentFloatRef(x) == ComparableBigExponentFloatRef(y)
}

fn same_pair(x: &(BigExponentFloat, Ordering), y: &(BigExponentFloat, Ordering)) -> bool {
    same(&x.0, &y.0) && x.1 == y.1
}

fn same3(x: &(BigExponentFloat, Ordering, bool), y: &(BigExponentFloat, Ordering, bool)) -> bool {
    same(&x.0, &y.0) && x.1 == y.1 && x.2 == y.2
}

fn round_to_integer_prec_round_all(
    x: &BigExponentFloat,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering, bool) {
    let r = x.clone().round_to_integer_prec_round(prec, rm);
    assert!(r.0.is_valid());
    assert!(same3(&x.round_to_integer_prec_round_ref(prec, rm), &r));
    if rm == Nearest {
        assert!(same3(&x.clone().round_to_integer_prec(prec), &r));
        assert!(same3(&x.round_to_integer_prec_ref(prec), &r));
    }
    r
}

fn round_to_integer_round_all(
    x: &BigExponentFloat,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering, bool) {
    let r = x.clone().round_to_integer_round(rm);
    assert!(r.0.is_valid());
    assert!(same3(&x.round_to_integer_round_ref(rm), &r));
    if rm == Nearest {
        assert!(same3(&x.clone().round_to_integer(), &r));
        assert!(same3(&x.round_to_integer_ref(), &r));
    }
    r
}

#[test]
fn test_round_to_integer() {
    let test_round = |x: &str,
                      x_hex: &str,
                      rm: RoundingMode,
                      out: &str,
                      out_hex: &str,
                      o: Ordering,
                      exact: bool| {
        assert_eq!(parse_big_exponent_float_hex(x_hex).to_string(), x);
        let x = &parse_big_exponent_float_hex(x_hex);
        assert_eq!(parse_big_exponent_float_hex(out_hex).to_string(), out);
        let out = &parse_big_exponent_float_hex(out_hex);
        assert!(same3(
            &round_to_integer_round_all(x, rm),
            &(out.clone(), o, exact)
        ));
    };
    let test_prec_round = |x: &str,
                           x_hex: &str,
                           prec: u64,
                           rm: RoundingMode,
                           out: &str,
                           out_hex: &str,
                           o: Ordering,
                           exact: bool| {
        assert_eq!(parse_big_exponent_float_hex(x_hex).to_string(), x);
        let x = &parse_big_exponent_float_hex(x_hex);
        assert_eq!(parse_big_exponent_float_hex(out_hex).to_string(), out);
        let out = &parse_big_exponent_float_hex(out_hex);
        assert!(same3(
            &round_to_integer_prec_round_all(x, prec, rm),
            &(out.clone(), o, exact)
        ));
    };
    test_round("NaN", "NaN", Floor, "NaN", "NaN", Equal, false);
    test_round(
        "Infinity", "Infinity", Floor, "Infinity", "Infinity", Equal, false,
    );
    test_round("0.0", "0x0.0", Floor, "0.0", "0x0.0", Equal, true);
    let x = BigExponentFloat::from(-2.5);
    test_round("-2.5", "-0x2.8", Floor, "-3.0", "-0x3.0", Less, false);
    test_round("-2.5", "-0x2.8", Ceiling, "-2.0", "-0x2.0", Greater, false);
    test_round("-2.5", "-0x2.8", Down, "-2.0", "-0x2.0", Greater, false);
    test_round("-2.5", "-0x2.8", Up, "-3.0", "-0x3.0", Less, false);
    test_round("-2.5", "-0x2.8", Nearest, "-2.0", "-0x2.0", Greater, false);
    assert!(same3(
        &x.round_to_integer_ties_away(),
        &(BigExponentFloat::from(-3), Less, false)
    ));
    // A single rounding: the nearest integer to 10.5 representable with 2 bits is 12, while
    // rounding to an integer first gives 10, which rounds to 8.
    let x = BigExponentFloat::from(10.5);
    test_prec_round("10.5", "0xa.8", 2, Nearest, "12.0", "0xc.0", Greater, false);
    assert!(same_pair(
        &x.round_to_integer_then_prec_round_ref(Nearest, 2, Nearest),
        &(BigExponentFloat::from(8), Less)
    ));
    // 7 is an integer, but needs rounding to fit 2 bits.
    test_prec_round("7.0", "0x7.0", 2, Nearest, "8.0", "0x8.0", Greater, true);
    // 2.5 at 2 bits: ties to even give 2, ties away 3.
    let x = BigExponentFloat::from(2.5);
    test_prec_round("2.5", "0x2.8", 2, Nearest, "2.0", "0x2.0", Less, false);
    assert!(same3(
        &x.round_to_integer_ties_away_prec_ref(2),
        &(BigExponentFloat::from(3), Greater, false)
    ));
    // 5 has 3 bits; at 2 bits it is a tie between 4 and 6: ties to even give 4, ties away 6.
    assert!(same3(
        &BigExponentFloat::from(5).round_to_integer_ties_away_prec(2),
        &(BigExponentFloat::from(6), Greater, true)
    ));
    test_prec_round("5.0", "0x5.0", 2, Nearest, "4.0", "0x4.0", Less, true);
    // Exponents beyond `Float`'s range: 2^-N rounds to 0 or 1, and 3 * 2^N is an integer, which
    // rounds to 2^(N + 2) at 1 bit, rounding up.
    test_round(
        "4.2e-30102999566398119522",
        "0x1.0E-25000000000000000000",
        Up,
        "1.0",
        "0x1.0",
        Greater,
        false,
    );
    test_round(
        "-4.2e-30102999566398119522",
        "-0x1.0E-25000000000000000000",
        Floor,
        "-1.0",
        "-0x1.0",
        Less,
        false,
    );
    test_round(
        "4.2e-30102999566398119522",
        "0x1.0E-25000000000000000000",
        Nearest,
        "0.0",
        "0x0.0",
        Less,
        false,
    );
    test_prec_round(
        "4.2e-30102999566398119522",
        "0x1.0E-25000000000000000000",
        10,
        Ceiling,
        "1.0",
        "0x1.0",
        Greater,
        false,
    );
    test_round(
        "7.1e30102999566398119521",
        "0x3.0E+25000000000000000000",
        Floor,
        "7.1e30102999566398119521",
        "0x3.0E+25000000000000000000",
        Equal,
        true,
    );
    test_prec_round(
        "7.1e30102999566398119521",
        "0x3.0E+25000000000000000000",
        1,
        Up,
        "9.5e30102999566398119521",
        "0x4.0E+25000000000000000000",
        Greater,
        true,
    );
}

#[test]
#[should_panic]
fn round_to_integer_round_fail() {
    BigExponentFloat::ONE.round_to_integer_round(Exact);
}

#[test]
#[should_panic]
fn round_to_integer_prec_round_fail() {
    BigExponentFloat::ONE.round_to_integer_prec_round(0, Floor);
}

// Whether `y` is an integer, or not finite.
fn is_integer_or_special(y: &BigExponentFloat) -> bool {
    !y.is_finite() || y.fractional_part_ref() == 0u32
}

#[test]
fn round_to_integer_prec_round_properties() {
    big_exponent_float_unsigned_rounding_mode_triple_gen_var_7().test_properties(
        |(x, prec, rm)| {
            let r = round_to_integer_prec_round_all(&x, prec, rm);
            assert!(is_integer_or_special(&r.0));
            assert_eq!(
                r.2,
                is_integer_or_special(&x) && !x.is_nan() && !x.is_infinite()
            );
            assert!(r.0.get_prec().is_none_or(|p| p <= prec));
            if r.1 == Equal {
                assert!(same(&r.0, &x));
            }
            // The result is correctly rounded among the integers with at most `prec` bits: it lies
            // in the right direction, and no such integer lies strictly between it and x.
            if x.is_finite() && big_exponent_float_exponent_is_small(&x) && rm != Nearest {
                let q = Rational::exact_from(&x);
                assert_eq!(Rational::exact_from(&r.0).cmp(&q), r.1);
            }
            let then = x.round_to_integer_then_prec_round_ref(Down, prec, rm);
            if x.is_finite() && !r.2 {
                assert!(then.1 != Equal || then.0.get_prec().is_none_or(|p| p <= prec));
            }
            let ties = x.round_to_integer_ties_away_prec_ref(prec);
            assert!(is_integer_or_special(&ties.0));
        },
    );
}

#[test]
fn round_to_integer_round_properties() {
    big_exponent_float_rounding_mode_pair_gen_var_1().test_properties(|(x, rm)| {
        if rm == Exact {
            return;
        }
        let r = round_to_integer_round_all(&x, rm);
        assert!(is_integer_or_special(&r.0));
        if x.is_finite() {
            let q = Rational::exact_from(&x);
            let expected = Integer::rounding_from(&q, rm).0;
            if rm != Nearest || (&q - Rational::from(&expected)).abs() * Rational::TWO != 1u32 {
                assert_eq!(Rational::exact_from(&r.0), Rational::from(&expected));
            }
            assert_eq!(Rational::exact_from(&r.0).cmp(&q), r.1);
            // Ties to even or away.
            let away = x.round_to_integer_ties_away_ref();
            assert!((Rational::exact_from(&away.0) - &q).abs() * Rational::TWO <= 1u32);
            // The two-step function with a precision large enough is the exact rounding.
            for irm in exhaustive_rounding_modes().filter(|&m| m != Exact) {
                let exact = x.round_to_integer_round_ref(irm).0;
                let prec = exact.get_prec().unwrap_or(1);
                assert!(same_pair(
                    &x.round_to_integer_then_prec_round_ref(irm, prec, Exact),
                    &(exact, Equal)
                ));
            }
        }
    });
}

#[test]
fn round_to_integer_float_agreement_properties() {
    big_exponent_float_unsigned_rounding_mode_triple_gen_var_7_float().test_properties(
        |((f, prec, rm), (x, _, _))| {
            let fr = f.round_to_integer_prec_round_ref(prec, rm);
            let r = round_to_integer_prec_round_all(&x, prec, rm);
            if fr.0.is_finite() || x.is_infinite() || x.is_nan() {
                assert!(same3(&r, &(float_to_big_exponent_float(&fr.0), fr.1, fr.2)));
            }
            let fr = f.round_to_integer_ties_away_prec_ref(prec);
            let r = x.round_to_integer_ties_away_prec_ref(prec);
            if fr.0.is_finite() || x.is_infinite() || x.is_nan() {
                assert!(same3(&r, &(float_to_big_exponent_float(&fr.0), fr.1, fr.2)));
            }
            for irm in exhaustive_rounding_modes().filter(|&m| m != Exact) {
                let (fz, fo) = f.round_to_integer_then_prec_round_ref(irm, prec, rm);
                if fz.is_finite() || x.is_infinite() || x.is_nan() {
                    assert!(same_pair(
                        &x.round_to_integer_then_prec_round_ref(irm, prec, rm),
                        &(float_to_big_exponent_float(&fz), fo)
                    ));
                }
            }
            let (fz, fo) = f.round_to_integer_ties_away_then_prec_round_ref(prec, rm);
            if fz.is_finite() || x.is_infinite() || x.is_nan() {
                assert!(same_pair(
                    &x.round_to_integer_ties_away_then_prec_round_ref(prec, rm),
                    &(float_to_big_exponent_float(&fz), fo)
                ));
            }
        },
    );
    big_exponent_float_gen_var_1_float().test_properties(|(f, x)| {
        for rm in exhaustive_rounding_modes().filter(|&m| m != Exact) {
            let fr = f.round_to_integer_round_ref(rm);
            if fr.0.is_finite() || x.is_infinite() || x.is_nan() {
                assert!(same3(
                    &round_to_integer_round_all(&x, rm),
                    &(float_to_big_exponent_float(&fr.0), fr.1, fr.2)
                ));
            }
        }
        let fr = f.round_to_integer_ties_away_ref();
        if fr.0.is_finite() || x.is_infinite() || x.is_nan() {
            assert!(same3(
                &x.round_to_integer_ties_away_ref(),
                &(float_to_big_exponent_float(&fr.0), fr.1, fr.2)
            ));
        }
    });
}
