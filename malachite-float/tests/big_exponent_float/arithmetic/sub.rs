// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::basic::traits::{Infinity, NaN, One, Zero};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_float::big_exponent_float::ComparableBigExponentFloatRef;
use malachite_float::big_exponent_float::arithmetic::add::add_prec_round_generic;
use malachite_float::test_util::big_exponent_float::{
    big_exponent_float_exponent_is_small, big_exponent_float_negate, big_exponent_float_power_of_2,
    float_to_big_exponent_float, parse_big_exponent_float_hex,
};
use malachite_float::test_util::generators::{
    big_exponent_float_big_exponent_float_unsigned_rounding_mode_quadruple_gen_var_3,
    big_exponent_float_pair_gen, float_float_unsigned_rounding_mode_quadruple_gen_var_2,
};
use malachite_float::{BigExponentFloat, Float};
use malachite_nz::integer::Integer;
use malachite_q::Rational;
use std::cmp::Ordering::{self, *};

fn same(x: &BigExponentFloat, y: &BigExponentFloat) -> bool {
    ComparableBigExponentFloatRef(x) == ComparableBigExponentFloatRef(y)
}

fn same_pair(x: &(BigExponentFloat, Ordering), y: &(BigExponentFloat, Ordering)) -> bool {
    same(&x.0, &y.0) && x.1 == y.1
}

// Whether a `Float` result is one that a `BigExponentFloat` result must match: not an overflow or
// underflow, which a `BigExponentFloat` never has.
fn float_result_comparable(x: &Float, o: Ordering) -> bool {
    x.get_exponent().map_or_else(
        || !x.is_infinite() && (!x.is_zero() || o == Equal),
        |e| e > Float::MIN_EXPONENT + 1 && e < Float::MAX_EXPONENT - 1,
    )
}

// Every by-value, by-reference, and in-place form of `sub_prec_round` and `sub_prec` agrees, and
// the result is valid.
fn sub_prec_round_all(
    x: &BigExponentFloat,
    y: &BigExponentFloat,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = x.clone().sub_prec_round(y.clone(), prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(
        &x.clone().sub_prec_round_val_ref(y, prec, rm),
        &r
    ));
    assert!(same_pair(
        &x.sub_prec_round_ref_val(y.clone(), prec, rm),
        &r
    ));
    assert!(same_pair(&x.sub_prec_round_ref_ref(y, prec, rm), &r));
    let mut z = x.clone();
    let o = z.sub_prec_round_assign(y.clone(), prec, rm);
    assert!(same_pair(&(z, o), &r));
    let mut z = x.clone();
    let o = z.sub_prec_round_assign_ref(y, prec, rm);
    assert!(same_pair(&(z, o), &r));
    if rm == Nearest {
        assert!(same_pair(&x.clone().sub_prec(y.clone(), prec), &r));
        assert!(same_pair(&x.clone().sub_prec_val_ref(y, prec), &r));
        assert!(same_pair(&x.sub_prec_ref_val(y.clone(), prec), &r));
        assert!(same_pair(&x.sub_prec_ref_ref(y, prec), &r));
        let mut z = x.clone();
        let o = z.sub_prec_assign(y.clone(), prec);
        assert!(same_pair(&(z, o), &r));
        let mut z = x.clone();
        let o = z.sub_prec_assign_ref(y, prec);
        assert!(same_pair(&(z, o), &r));
    }
    r
}

// Every form of the exact `-` agrees, and the result is valid.
fn sub_all(x: &BigExponentFloat, y: &BigExponentFloat) -> BigExponentFloat {
    let r = x.clone() - y.clone();
    assert!(r.is_valid());
    assert!(same(&(x.clone() - y), &r));
    assert!(same(&(x - y.clone()), &r));
    assert!(same(&(x - y), &r));
    let mut z = x.clone();
    z -= y.clone();
    assert!(same(&z, &r));
    let mut z = x.clone();
    z -= y;
    assert!(same(&z, &r));
    r
}

#[test]
fn test_sub() {
    let big = Integer::from_str("100000000000000000000").unwrap();
    let huge = big_exponent_float_power_of_2(big.clone());
    let one = BigExponentFloat::ONE;
    let nan = BigExponentFloat::NAN;
    let inf = BigExponentFloat::INFINITY;
    let zero = BigExponentFloat::ZERO;
    let check = |x: &str,
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
        let r = sub_prec_round_all(x, y, prec, rm);
        assert!(
            same(&r.0, out),
            "{x:?} {y:?} {prec} {rm}: {:?} != {out:?}",
            r.0
        );
        assert_eq!(r.1, o);
    };

    check("NaN", "NaN", "1.0", "0x1.0", 10, Floor, "NaN", "NaN", Equal);
    check(
        "Infinity", "Infinity", "Infinity", "Infinity", 10, Floor, "NaN", "NaN", Equal,
    );
    check(
        "Infinity",
        "Infinity",
        "-Infinity",
        "-Infinity",
        10,
        Floor,
        "Infinity",
        "Infinity",
        Equal,
    );
    check(
        "1.0",
        "0x1.0",
        "Infinity",
        "Infinity",
        10,
        Floor,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    check(
        "0.0", "0x0.0", "0.0", "0x0.0", 10, Floor, "0.0", "0x0.0", Equal,
    );
    check(
        "0.0",
        "0x0.0",
        "3.1415926535897931",
        "0x3.243f6a8885a3",
        53,
        Exact,
        "-3.1415926535897931",
        "-0x3.243f6a8885a3",
        Equal,
    );
    check(
        "3.1415926535897931",
        "0x3.243f6a8885a3",
        "3.1415926535897931",
        "0x3.243f6a8885a3",
        10,
        Floor,
        "0.0",
        "0x0.0",
        Equal,
    );
    check(
        "2.4e30102999566398119521",
        "0x1.0E+25000000000000000000",
        "2.4e30102999566398119521",
        "0x1.0E+25000000000000000000",
        10,
        Ceiling,
        "0.0",
        "0x0.0",
        Equal,
    );
    // Cancellation at a huge exponent.
    check(
        "2.3653176496878716463137939216895230994454804444359351664030925e30102999566398119521",
        "0x1.00000000000000000000000000000000000000000000000001E+25000000000000000000",
        "2.4e30102999566398119521",
        "0x1.0E+25000000000000000000",
        1,
        Exact,
        "1.5e30102999566398119461",
        "0x1.0E+24999999999999999950",
        Equal,
    );
    check(
        "1.0",
        "0x1.0",
        "4.2e-30102999566398119522",
        "0x1.0E-25000000000000000000",
        10,
        Floor,
        "0.99902",
        "0x0.ffc",
        Less,
    );
    check(
        "1.0",
        "0x1.0",
        "4.2e-30102999566398119522",
        "0x1.0E-25000000000000000000",
        10,
        Ceiling,
        "1.0",
        "0x1.0",
        Greater,
    );
    check(
        "1.0",
        "0x1.0",
        "4.2e-30102999566398119522",
        "0x1.0E-25000000000000000000",
        10,
        Nearest,
        "1.0",
        "0x1.0",
        Greater,
    );
    // Rounding toward zero lands just below huge in absolute value.
    check(
        "4.2e-30102999566398119522",
        "0x1.0E-25000000000000000000",
        "2.4e30102999566398119521",
        "0x1.0E+25000000000000000000",
        10,
        Down,
        "-2.3630e30102999566398119521",
        "-0xf.fcE+24999999999999999999",
        Greater,
    );
    check(
        "3.1415926535897931",
        "0x3.243f6a8885a3",
        "2.7182818284590451",
        "0x2.b7e151628aed2",
        5,
        Floor,
        "0.422",
        "0x0.6c",
        Less,
    );
    check(
        "3.1415926535897931",
        "0x3.243f6a8885a3",
        "2.7182818284590451",
        "0x2.b7e151628aed2",
        5,
        Ceiling,
        "0.44",
        "0x0.7",
        Greater,
    );
    let d = sub_all(&one, &(&one >> 100u32));
    assert_eq!(d.get_prec(), Some(100));
    assert!(same(&sub_all(&huge, &huge), &zero));
    assert!(same(&sub_all(&inf, &inf), &nan));
}

#[test]
#[should_panic]
fn sub_prec_round_fail_1() {
    BigExponentFloat::ONE.sub_prec_round(BigExponentFloat::ONE, 0, Floor);
}

#[test]
#[should_panic]
fn sub_prec_round_fail_2() {
    let x = BigExponentFloat::from(1.5);
    let y = BigExponentFloat::from(1.0 / 3.0);
    x.sub_prec_round(y, 1, Exact);
}

#[test]
#[should_panic]
fn sub_prec_fail() {
    BigExponentFloat::ONE.sub_prec(BigExponentFloat::ONE, 0);
}

#[test]
fn sub_prec_round_properties() {
    big_exponent_float_big_exponent_float_unsigned_rounding_mode_quadruple_gen_var_3()
        .test_properties(|(x, y, prec, rm)| {
            let r = sub_prec_round_all(&x, &y, prec, rm);
            // The kernels agree with the kernel-free implementation, at any exponents.
            assert!(same_pair(
                &add_prec_round_generic(&x, &y, true, prec, rm),
                &r
            ));
            if r.0.is_nan() || r.0.is_zero() || r.0.is_infinite() {
                assert_eq!(r.1, Equal);
            } else {
                assert!(r.0.get_prec().unwrap() <= prec);
            }
            // -(x - y) = y - x, with the rounding mode and the ordering reversed.
            let (s, o) = y.sub_prec_round_ref_ref(&x, prec, -rm);
            assert!(same(&big_exponent_float_negate(s), &r.0));
            assert_eq!(o.reverse(), r.1);
            assert!(same_pair(
                &x.add_prec_round_ref_val(big_exponent_float_negate(y.clone()), prec, rm),
                &r
            ));
            if big_exponent_float_exponent_is_small(&x) && big_exponent_float_exponent_is_small(&y)
            {
                // Rounding the exact result gives the same value and ordering.
                let exact = sub_all(&x, &y);
                assert!(same_pair(&exact.round_to_prec_round_ref(prec, rm), &r));
                if exact.is_finite() {
                    let q = Rational::exact_from(&x) - Rational::exact_from(&y);
                    assert_eq!(Rational::exact_from(&exact), q);
                    assert_eq!(Rational::exact_from(&r.0).cmp(&q), r.1);
                }
            }
        });
}

#[test]
fn sub_properties() {
    big_exponent_float_pair_gen().test_properties(|(x, y)| {
        if big_exponent_float_exponent_is_small(&x) && big_exponent_float_exponent_is_small(&y) {
            let r = sub_all(&x, &y);
            if let (Ok(qx), Ok(qy)) = (Rational::try_from(&x), Rational::try_from(&y)) {
                assert_eq!(Rational::exact_from(&r), qx - qy);
            }
        }
    });
}

#[test]
fn sub_prec_round_float_agreement_properties() {
    float_float_unsigned_rounding_mode_quadruple_gen_var_2().test_properties(|(x, y, prec, rm)| {
        let (f, fo) = x.sub_prec_round_ref_ref(&y, prec, rm);
        if float_result_comparable(&f, fo) {
            let bx = float_to_big_exponent_float(&x);
            let by = float_to_big_exponent_float(&y);
            let r = sub_prec_round_all(&bx, &by, prec, rm);
            assert!(same(&r.0, &float_to_big_exponent_float(&f)));
            assert_eq!(r.1, fo);
        }
    });
}
