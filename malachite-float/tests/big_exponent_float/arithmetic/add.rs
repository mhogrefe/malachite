// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity, One, Zero};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_float::big_exponent_float::ComparableBigExponentFloatRef;
use malachite_float::big_exponent_float::arithmetic::add::add_prec_round_generic;
use malachite_float::test_util::big_exponent_float::{
    big_exponent_float_exponent_is_small, big_exponent_float_negate, big_exponent_float_power_of_2,
    float_to_big_exponent_float, parse_big_exponent_float_hex,
};
use malachite_float::test_util::generators::{
    big_exponent_float_big_exponent_float_unsigned_rounding_mode_quadruple_gen_var_2,
    big_exponent_float_pair_gen, float_float_unsigned_rounding_mode_quadruple_gen_var_1,
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

// Every by-value, by-reference, and in-place form of `add_prec_round` and `add_prec` agrees, and
// the result is valid.
fn add_prec_round_all(
    x: &BigExponentFloat,
    y: &BigExponentFloat,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = x.clone().add_prec_round(y.clone(), prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(
        &x.clone().add_prec_round_val_ref(y, prec, rm),
        &r
    ));
    assert!(same_pair(
        &x.add_prec_round_ref_val(y.clone(), prec, rm),
        &r
    ));
    assert!(same_pair(&x.add_prec_round_ref_ref(y, prec, rm), &r));
    let mut z = x.clone();
    let o = z.add_prec_round_assign(y.clone(), prec, rm);
    assert!(same_pair(&(z, o), &r));
    let mut z = x.clone();
    let o = z.add_prec_round_assign_ref(y, prec, rm);
    assert!(same_pair(&(z, o), &r));
    if rm == Nearest {
        assert!(same_pair(&x.clone().add_prec(y.clone(), prec), &r));
        assert!(same_pair(&x.clone().add_prec_val_ref(y, prec), &r));
        assert!(same_pair(&x.add_prec_ref_val(y.clone(), prec), &r));
        assert!(same_pair(&x.add_prec_ref_ref(y, prec), &r));
        let mut z = x.clone();
        let o = z.add_prec_assign(y.clone(), prec);
        assert!(same_pair(&(z, o), &r));
        let mut z = x.clone();
        let o = z.add_prec_assign_ref(y, prec);
        assert!(same_pair(&(z, o), &r));
    }
    r
}

// Every form of the exact `+` agrees, and the result is valid.
fn add_all(x: &BigExponentFloat, y: &BigExponentFloat) -> BigExponentFloat {
    let r = x.clone() + y.clone();
    assert!(r.is_valid());
    assert!(same(&(x.clone() + y), &r));
    assert!(same(&(x + y.clone()), &r));
    assert!(same(&(x + y), &r));
    let mut z = x.clone();
    z += y.clone();
    assert!(same(&z, &r));
    let mut z = x.clone();
    z += y;
    assert!(same(&z, &r));
    r
}

#[test]
fn test_add() {
    let big = Integer::from_str("100000000000000000000").unwrap();
    let huge = big_exponent_float_power_of_2(big.clone());
    let one = BigExponentFloat::ONE;
    let pi = parse_big_exponent_float_hex("0x3.243f6a8885a30#53");
    let nan = BigExponentFloat::NAN;
    let inf = BigExponentFloat::INFINITY;
    let ninf = BigExponentFloat::NEGATIVE_INFINITY;
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
        let r = add_prec_round_all(x, y, prec, rm);
        assert!(
            same(&r.0, out),
            "{x:?} {y:?} {prec} {rm}: {:?} != {out:?}",
            r.0
        );
        assert_eq!(r.1, o);
    };

    check("NaN", "NaN", "1.0", "0x1.0", 10, Floor, "NaN", "NaN", Equal);
    check(
        "Infinity",
        "Infinity",
        "-Infinity",
        "-Infinity",
        10,
        Floor,
        "NaN",
        "NaN",
        Equal,
    );
    check(
        "Infinity", "Infinity", "1.0", "0x1.0", 10, Floor, "Infinity", "Infinity", Equal,
    );
    check(
        "1.0",
        "0x1.0",
        "-Infinity",
        "-Infinity",
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
        "3.1415926535897931",
        "0x3.243f6a8885a3",
        "0.0",
        "0x0.0",
        53,
        Exact,
        "3.1415926535897931",
        "0x3.243f6a8885a3",
        Equal,
    );
    check(
        "0.0",
        "0x0.0",
        "3.1415926535897931",
        "0x3.243f6a8885a3",
        1,
        Floor,
        "2.0",
        "0x2.0",
        Less,
    );
    // x + (-x) is 0, which has no sign, in every rounding mode.
    check(
        "3.1415926535897931",
        "0x3.243f6a8885a3",
        "-3.1415926535897931",
        "-0x3.243f6a8885a3",
        10,
        Floor,
        "0.0",
        "0x0.0",
        Equal,
    );
    check(
        "2.4e30102999566398119521",
        "0x1.0E+25000000000000000000",
        "-2.4e30102999566398119521",
        "-0x1.0E+25000000000000000000",
        10,
        Ceiling,
        "0.0",
        "0x0.0",
        Equal,
    );
    // A summand far below the other only affects the rounding.
    check(
        "1.0",
        "0x1.0",
        "4.2e-30102999566398119522",
        "0x1.0E-25000000000000000000",
        10,
        Floor,
        "1.0",
        "0x1.0",
        Less,
    );
    check(
        "1.0",
        "0x1.0",
        "4.2e-30102999566398119522",
        "0x1.0E-25000000000000000000",
        10,
        Ceiling,
        "1.0020",
        "0x1.008",
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
        Less,
    );
    check(
        "1.0",
        "0x1.0",
        "-4.2e-30102999566398119522",
        "-0x1.0E-25000000000000000000",
        10,
        Floor,
        "0.99902",
        "0x0.ffc",
        Less,
    );
    check(
        "1.0",
        "0x1.0",
        "-4.2e-30102999566398119522",
        "-0x1.0E-25000000000000000000",
        10,
        Ceiling,
        "1.0",
        "0x1.0",
        Greater,
    );
    check(
        "4.2e-30102999566398119522",
        "0x1.0E-25000000000000000000",
        "2.4e30102999566398119521",
        "0x1.0E+25000000000000000000",
        10,
        Down,
        "2.4e30102999566398119521",
        "0x1.0E+25000000000000000000",
        Less,
    );
    check(
        "4.2e-30102999566398119522",
        "0x1.0E-25000000000000000000",
        "2.4e30102999566398119521",
        "0x1.0E+25000000000000000000",
        10,
        Up,
        "2.3699e30102999566398119521",
        "0x1.008E+25000000000000000000",
        Greater,
    );
    // Sums at huge exponents are exact when they fit.
    check(
        "2.4e30102999566398119521",
        "0x1.0E+25000000000000000000",
        "2.4e30102999566398119521",
        "0x1.0E+25000000000000000000",
        1,
        Exact,
        "4.7e30102999566398119521",
        "0x2.0E+25000000000000000000",
        Equal,
    );
    check(
        "4.2e-30102999566398119522",
        "0x1.0E-25000000000000000000",
        "4.2e-30102999566398119522",
        "0x1.0E-25000000000000000000",
        1,
        Exact,
        "8.5e-30102999566398119522",
        "0x2.0E-25000000000000000000",
        Equal,
    );
    check(
        "3.1415926535897931",
        "0x3.243f6a8885a3",
        "2.7182818284590451",
        "0x2.b7e151628aed2",
        5,
        Floor,
        "5.75",
        "0x5.c",
        Less,
    );
    check(
        "3.1415926535897931",
        "0x3.243f6a8885a3",
        "2.7182818284590451",
        "0x2.b7e151628aed2",
        5,
        Ceiling,
        "6.0",
        "0x6.0",
        Greater,
    );
    check(
        "3.1415926535897931",
        "0x3.243f6a8885a3",
        "2.7182818284590451",
        "0x2.b7e151628aed2",
        20,
        Nearest,
        "5.8598709",
        "0x5.dc208",
        Less,
    );
    // The exact sum spans the gap between the exponents.
    let s = add_all(&one, &(&one >> 100u32));
    assert_eq!(s.get_prec(), Some(101));
    assert!(same(&add_all(&huge, &huge), &(&huge << 1u32)));
    assert!(same(&add_all(&nan, &one), &nan));
    assert!(same(&add_all(&inf, &ninf), &nan));
    assert!(same(
        &add_all(&pi, &big_exponent_float_negate(pi.clone())),
        &zero
    ));
}

#[test]
#[should_panic]
fn add_prec_round_fail_1() {
    BigExponentFloat::ONE.add_prec_round(BigExponentFloat::ONE, 0, Floor);
}

#[test]
#[should_panic]
fn add_prec_round_fail_2() {
    let x = BigExponentFloat::from(1.5);
    let y = BigExponentFloat::from(1.0 / 3.0);
    x.add_prec_round(y, 1, Exact);
}

#[test]
#[should_panic]
fn add_prec_fail() {
    BigExponentFloat::ONE.add_prec(BigExponentFloat::ONE, 0);
}

#[test]
fn add_prec_round_properties() {
    big_exponent_float_big_exponent_float_unsigned_rounding_mode_quadruple_gen_var_2()
        .test_properties(|(x, y, prec, rm)| {
            let r = add_prec_round_all(&x, &y, prec, rm);
            // The kernels agree with the kernel-free implementation, at any exponents.
            assert!(same_pair(
                &add_prec_round_generic(&x, &y, false, prec, rm),
                &r
            ));
            if r.0.is_nan() || r.0.is_zero() || r.0.is_infinite() {
                assert_eq!(r.1, Equal);
            } else {
                assert!(r.0.get_prec().unwrap() <= prec);
            }
            // -(x + y) = (-x) + (-y), with the rounding mode and the ordering reversed.
            let (s, o) = big_exponent_float_negate(x.clone()).add_prec_round(
                big_exponent_float_negate(y.clone()),
                prec,
                -rm,
            );
            assert!(same(&big_exponent_float_negate(s), &r.0));
            assert_eq!(o.reverse(), r.1);
            // x - y = x + (-y), and x + y = y + x.
            assert!(same_pair(
                &x.sub_prec_round_ref_val(big_exponent_float_negate(y.clone()), prec, rm),
                &r
            ));
            assert!(same_pair(&y.add_prec_round_ref_ref(&x, prec, rm), &r));
            if big_exponent_float_exponent_is_small(&x) && big_exponent_float_exponent_is_small(&y)
            {
                // Rounding the exact result gives the same value and ordering.
                let exact = add_all(&x, &y);
                assert!(same_pair(&exact.round_to_prec_round_ref(prec, rm), &r));
                if exact.is_finite() {
                    let q = Rational::exact_from(&x) + Rational::exact_from(&y);
                    assert_eq!(Rational::exact_from(&exact), q);
                    assert_eq!(Rational::exact_from(&r.0).cmp(&q), r.1);
                }
            }
        });
}

#[test]
fn add_properties() {
    big_exponent_float_pair_gen().test_properties(|(x, y)| {
        if big_exponent_float_exponent_is_small(&x) && big_exponent_float_exponent_is_small(&y) {
            let r = add_all(&x, &y);
            if let (Ok(qx), Ok(qy)) = (Rational::try_from(&x), Rational::try_from(&y)) {
                assert_eq!(Rational::exact_from(&r), qx + qy);
            }
        }
    });
}

#[test]
fn add_prec_round_float_agreement_properties() {
    float_float_unsigned_rounding_mode_quadruple_gen_var_1().test_properties(|(x, y, prec, rm)| {
        let (f, fo) = x.add_prec_round_ref_ref(&y, prec, rm);
        if float_result_comparable(&f, fo) {
            let bx = float_to_big_exponent_float(&x);
            let by = float_to_big_exponent_float(&y);
            let r = add_prec_round_all(&bx, &by, prec, rm);
            assert!(same(&r.0, &float_to_big_exponent_float(&f)));
            assert_eq!(r.1, fo);
        }
    });
}
