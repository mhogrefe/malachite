// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{AddMul, AddMulAssign};
use malachite_base::num::basic::traits::One;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_float::big_exponent_float::ComparableBigExponentFloatRef;
use malachite_float::big_exponent_float::arithmetic::add_mul::{
    add_mul_prec_round_generic, add_mul_rational_prec_round_generic,
};
use malachite_float::test_util::big_exponent_float::{
    big_exponent_float_exponent_is_small, float_to_big_exponent_float, parse_big_exponent_float_hex,
};
use malachite_float::test_util::generators::*;
use malachite_float::{BigExponentFloat, Float};
use malachite_q::Rational;
use std::cmp::Ordering::{self, *};
use std::str::FromStr;

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

fn to_bef(x: &Float) -> BigExponentFloat {
    float_to_big_exponent_float(x)
}

fn add_mul_prec_round_all(
    x: &BigExponentFloat,
    y: &BigExponentFloat,
    z: &BigExponentFloat,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = x.clone().add_mul_prec_round(y.clone(), z.clone(), prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(
        &x.clone()
            .add_mul_prec_round_val_val_ref(y.clone(), z, prec, rm),
        &r
    ));
    assert!(same_pair(
        &x.clone()
            .add_mul_prec_round_val_ref_val(y, z.clone(), prec, rm),
        &r
    ));
    assert!(same_pair(
        &x.clone().add_mul_prec_round_val_ref_ref(y, z, prec, rm),
        &r
    ));
    assert!(same_pair(
        &x.add_mul_prec_round_ref_val_val(y.clone(), z.clone(), prec, rm),
        &r
    ));
    assert!(same_pair(
        &x.add_mul_prec_round_ref_val_ref(y.clone(), z, prec, rm),
        &r
    ));
    assert!(same_pair(
        &x.add_mul_prec_round_ref_ref_val(y, z.clone(), prec, rm),
        &r
    ));
    assert!(same_pair(
        &x.add_mul_prec_round_ref_ref_ref(y, z, prec, rm),
        &r
    ));
    let mut t = x.clone();
    let o = t.add_mul_prec_round_assign(y.clone(), z.clone(), prec, rm);
    assert!(same_pair(&(t, o), &r));
    let mut t = x.clone();
    let o = t.add_mul_prec_round_assign_val_ref(y.clone(), z, prec, rm);
    assert!(same_pair(&(t, o), &r));
    let mut t = x.clone();
    let o = t.add_mul_prec_round_assign_ref_val(y, z.clone(), prec, rm);
    assert!(same_pair(&(t, o), &r));
    let mut t = x.clone();
    let o = t.add_mul_prec_round_assign_ref_ref(y, z, prec, rm);
    assert!(same_pair(&(t, o), &r));
    if rm == Nearest {
        assert!(same_pair(
            &x.clone().add_mul_prec(y.clone(), z.clone(), prec),
            &r
        ));
        assert!(same_pair(
            &x.clone().add_mul_prec_val_val_ref(y.clone(), z, prec),
            &r
        ));
        assert!(same_pair(
            &x.clone().add_mul_prec_val_ref_val(y, z.clone(), prec),
            &r
        ));
        assert!(same_pair(
            &x.clone().add_mul_prec_val_ref_ref(y, z, prec),
            &r
        ));
        assert!(same_pair(
            &x.add_mul_prec_ref_val_val(y.clone(), z.clone(), prec),
            &r
        ));
        assert!(same_pair(
            &x.add_mul_prec_ref_val_ref(y.clone(), z, prec),
            &r
        ));
        assert!(same_pair(
            &x.add_mul_prec_ref_ref_val(y, z.clone(), prec),
            &r
        ));
        assert!(same_pair(&x.add_mul_prec_ref_ref_ref(y, z, prec), &r));
        let mut t = x.clone();
        let o = t.add_mul_prec_assign(y.clone(), z.clone(), prec);
        assert!(same_pair(&(t, o), &r));
        let mut t = x.clone();
        let o = t.add_mul_prec_assign_val_ref(y.clone(), z, prec);
        assert!(same_pair(&(t, o), &r));
        let mut t = x.clone();
        let o = t.add_mul_prec_assign_ref_val(y, z.clone(), prec);
        assert!(same_pair(&(t, o), &r));
        let mut t = x.clone();
        let o = t.add_mul_prec_assign_ref_ref(y, z, prec);
        assert!(same_pair(&(t, o), &r));
    }
    r
}

fn add_mul_all(
    x: &BigExponentFloat,
    y: &BigExponentFloat,
    z: &BigExponentFloat,
) -> BigExponentFloat {
    let r = x.clone().add_mul(y.clone(), z.clone());
    assert!(r.is_valid());
    assert!(same(&x.clone().add_mul(y.clone(), z), &r));
    assert!(same(&x.clone().add_mul(y, z.clone()), &r));
    assert!(same(&x.clone().add_mul(y, z), &r));
    assert!(same(&x.add_mul(y.clone(), z.clone()), &r));
    assert!(same(&x.add_mul(y.clone(), z), &r));
    assert!(same(&x.add_mul(y, z.clone()), &r));
    assert!(same(&x.add_mul(y, z), &r));
    let mut t = x.clone();
    t.add_mul_assign(y.clone(), z.clone());
    assert!(same(&t, &r));
    let mut t = x.clone();
    t.add_mul_assign(y.clone(), z);
    assert!(same(&t, &r));
    let mut t = x.clone();
    t.add_mul_assign(y, z.clone());
    assert!(same(&t, &r));
    let mut t = x.clone();
    t.add_mul_assign(y, z);
    assert!(same(&t, &r));
    r
}

#[test]
fn test_add_mul_prec_round() {
    let check = |x: &str,
                 x_hex: &str,
                 y: &str,
                 y_hex: &str,
                 z: &str,
                 z_hex: &str,
                 prec: u64,
                 rm: RoundingMode,
                 out: &str,
                 out_hex: &str,
                 o| {
        assert_eq!(parse_big_exponent_float_hex(x_hex).to_string(), x);
        let x = &parse_big_exponent_float_hex(x_hex);
        assert_eq!(parse_big_exponent_float_hex(y_hex).to_string(), y);
        let y = &parse_big_exponent_float_hex(y_hex);
        assert_eq!(parse_big_exponent_float_hex(z_hex).to_string(), z);
        let z = &parse_big_exponent_float_hex(z_hex);
        assert_eq!(parse_big_exponent_float_hex(out_hex).to_string(), out);
        let out = &parse_big_exponent_float_hex(out_hex);
        let r = add_mul_prec_round_all(x, y, z, prec, rm);
        assert!(same(&r.0, out), "{:?} != {out:?}", r.0);
        assert_eq!(r.1, o);
    };
    // The huge terms cancel exactly.
    check(
        "2.4e30102999566398119521",
        "0x1.0E+25000000000000000000",
        "-1.0",
        "-0x1.0",
        "2.4e30102999566398119521",
        "0x1.0E+25000000000000000000",
        10,
        Exact,
        "0.0",
        "0x0.0",
        Equal,
    );
    check(
        "1.0",
        "0x1.0",
        "2.4e30102999566398119521",
        "0x1.0E+25000000000000000000",
        "4.2e-30102999566398119522",
        "0x1.0E-25000000000000000000",
        1,
        Exact,
        "2.0",
        "0x2.0",
        Equal,
    );
    // A tiny x is far below yz.
    check(
        "4.2e-30102999566398119522",
        "0x1.0E-25000000000000000000",
        "2.4e30102999566398119521",
        "0x1.0E+25000000000000000000",
        "2.4e30102999566398119521",
        "0x1.0E+25000000000000000000",
        10,
        Floor,
        "5.6e60205999132796239042",
        "0x1.0E+50000000000000000000",
        Less,
    );
    check(
        "NaN", "NaN", "1.0", "0x1.0", "1.0", "0x1.0", 10, Floor, "NaN", "NaN", Equal,
    );
    check(
        "1.0", "0x1.0", "Infinity", "Infinity", "0.0", "0x0.0", 10, Floor, "NaN", "NaN", Equal,
    );
    check(
        "Infinity", "Infinity", "Infinity", "Infinity", "-1.0", "-0x1.0", 10, Floor, "NaN", "NaN",
        Equal,
    );
    check(
        "1.5", "0x1.8", "0.50", "0x0.8", "3.0", "0x3.0", 2, Exact, "3.0", "0x3.0", Equal,
    );
}

#[test]
#[should_panic]
fn add_mul_prec_round_fail_1() {
    BigExponentFloat::ONE.add_mul_prec_round(
        BigExponentFloat::ONE,
        BigExponentFloat::ONE,
        0,
        Floor,
    );
}

#[test]
#[should_panic]
fn add_mul_prec_round_fail_2() {
    BigExponentFloat::from(3.0).add_mul_prec_round(
        BigExponentFloat::from(3.0),
        BigExponentFloat::from(3.0),
        1,
        Exact,
    );
}

#[test]
fn add_mul_prec_round_properties() {
    let generator = big_exponent_float_big_exponent_float_big_exponent_float_unsigned_rounding_mode_quintuple_gen_var_1;
    generator().test_properties(|(x, y, z, prec, rm)| {
        let r = add_mul_prec_round_all(&x, &y, &z, prec, rm);
        // The kernels agree with the kernel-free implementation, at any exponents.
        assert!(same_pair(
            &add_mul_prec_round_generic(&x, &y, &z, false, prec, rm),
            &r
        ));
        if r.0.is_nan() || r.0.is_zero() || r.0.is_infinite() {
            assert_eq!(r.1, Equal);
        } else {
            assert!(r.0.get_prec().unwrap() <= prec);
        }
        // The product's factors commute.
        assert!(same_pair(
            &x.add_mul_prec_round_ref_ref_ref(&z, &y, prec, rm),
            &r
        ));
        if [&x, &y, &z]
            .iter()
            .all(|v| big_exponent_float_exponent_is_small(v))
            && [&x, &y, &z].iter().all(|v| v.is_finite())
        {
            let q = Rational::exact_from(&x) + Rational::exact_from(&y) * Rational::exact_from(&z);
            assert_eq!(Rational::exact_from(&r.0).cmp(&q), r.1);
        }
    });
}

#[test]
fn add_mul_properties() {
    big_exponent_float_triple_gen_var_1().test_properties(|(x, y, z)| {
        assert!(same(&add_mul_all(&x, &y, &z), &(&x + &(&y * &z))));
    });
}

#[test]
fn add_mul_prec_round_float_agreement_properties() {
    float_float_float_unsigned_rounding_mode_quintuple_gen_var_1().test_properties(
        |(x, y, z, prec, rm)| {
            let (f, fo) = x.add_mul_prec_round_ref_ref_ref(&y, &z, prec, rm);
            if float_result_comparable(&f, fo) {
                let r = add_mul_prec_round_all(&to_bef(&x), &to_bef(&y), &to_bef(&z), prec, rm);
                assert!(same(&r.0, &to_bef(&f)));
                assert_eq!(r.1, fo);
            }
        },
    );
}

fn add_mul_rational_prec_round_all(
    x: &BigExponentFloat,
    y: &BigExponentFloat,
    z: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = x
        .clone()
        .add_mul_rational_prec_round(y.clone(), z.clone(), prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(
        &x.clone()
            .add_mul_rational_prec_round_val_val_ref(y.clone(), z, prec, rm),
        &r
    ));
    assert!(same_pair(
        &x.clone()
            .add_mul_rational_prec_round_val_ref_val(y, z.clone(), prec, rm),
        &r
    ));
    assert!(same_pair(
        &x.clone()
            .add_mul_rational_prec_round_val_ref_ref(y, z, prec, rm),
        &r
    ));
    assert!(same_pair(
        &x.add_mul_rational_prec_round_ref_val_val(y.clone(), z.clone(), prec, rm),
        &r
    ));
    assert!(same_pair(
        &x.add_mul_rational_prec_round_ref_val_ref(y.clone(), z, prec, rm),
        &r
    ));
    assert!(same_pair(
        &x.add_mul_rational_prec_round_ref_ref_val(y, z.clone(), prec, rm),
        &r
    ));
    assert!(same_pair(
        &x.add_mul_rational_prec_round_ref_ref_ref(y, z, prec, rm),
        &r
    ));
    let mut a = x.clone();
    let o = a.add_mul_rational_prec_round_assign(y.clone(), z.clone(), prec, rm);
    assert!(same_pair(&(a, o), &r));
    let mut a = x.clone();
    let o = a.add_mul_rational_prec_round_assign_val_ref(y.clone(), z, prec, rm);
    assert!(same_pair(&(a, o), &r));
    let mut a = x.clone();
    let o = a.add_mul_rational_prec_round_assign_ref_val(y, z.clone(), prec, rm);
    assert!(same_pair(&(a, o), &r));
    let mut a = x.clone();
    let o = a.add_mul_rational_prec_round_assign_ref_ref(y, z, prec, rm);
    assert!(same_pair(&(a, o), &r));
    if rm == Nearest {
        assert!(same_pair(
            &x.clone().add_mul_rational_prec(y.clone(), z.clone(), prec),
            &r
        ));
        assert!(same_pair(
            &x.clone()
                .add_mul_rational_prec_val_val_ref(y.clone(), z, prec),
            &r
        ));
        assert!(same_pair(
            &x.clone()
                .add_mul_rational_prec_val_ref_val(y, z.clone(), prec),
            &r
        ));
        assert!(same_pair(
            &x.clone().add_mul_rational_prec_val_ref_ref(y, z, prec),
            &r
        ));
        assert!(same_pair(
            &x.add_mul_rational_prec_ref_val_val(y.clone(), z.clone(), prec),
            &r
        ));
        assert!(same_pair(
            &x.add_mul_rational_prec_ref_val_ref(y.clone(), z, prec),
            &r
        ));
        assert!(same_pair(
            &x.add_mul_rational_prec_ref_ref_val(y, z.clone(), prec),
            &r
        ));
        assert!(same_pair(
            &x.add_mul_rational_prec_ref_ref_ref(y, z, prec),
            &r
        ));
        let mut a = x.clone();
        let o = a.add_mul_rational_prec_assign(y.clone(), z.clone(), prec);
        assert!(same_pair(&(a, o), &r));
        let mut a = x.clone();
        let o = a.add_mul_rational_prec_assign_val_ref(y.clone(), z, prec);
        assert!(same_pair(&(a, o), &r));
        let mut a = x.clone();
        let o = a.add_mul_rational_prec_assign_ref_val(y, z.clone(), prec);
        assert!(same_pair(&(a, o), &r));
        let mut a = x.clone();
        let o = a.add_mul_rational_prec_assign_ref_ref(y, z, prec);
        assert!(same_pair(&(a, o), &r));
    }
    r
}

#[test]
fn test_add_mul_rational_prec_round() {
    let check = |x: &str,
                 x_hex: &str,
                 y: &str,
                 y_hex: &str,
                 z: &str,
                 prec: u64,
                 rm: RoundingMode,
                 out: &str,
                 out_hex: &str,
                 o| {
        assert_eq!(parse_big_exponent_float_hex(x_hex).to_string(), x);
        let x = &parse_big_exponent_float_hex(x_hex);
        assert_eq!(parse_big_exponent_float_hex(y_hex).to_string(), y);
        let y = &parse_big_exponent_float_hex(y_hex);
        assert_eq!(parse_big_exponent_float_hex(out_hex).to_string(), out);
        let out = &parse_big_exponent_float_hex(out_hex);
        let r = add_mul_rational_prec_round_all(x, y, &Rational::from_str(z).unwrap(), prec, rm);
        assert!(same(&r.0, out), "{:?} != {out:?}", r.0);
        assert_eq!(r.1, o);
    };
    check(
        "NaN", "NaN", "1.0", "0x1.0", "1/3", 10, Floor, "NaN", "NaN", Equal,
    );
    check(
        "1.0", "0x1.0", "NaN", "NaN", "1/3", 10, Floor, "NaN", "NaN", Equal,
    );
    check(
        "1.0", "0x1.0", "Infinity", "Infinity", "0", 10, Floor, "NaN", "NaN", Equal,
    );
    check(
        "-Infinity",
        "-Infinity",
        "Infinity",
        "Infinity",
        "1/3",
        10,
        Floor,
        "NaN",
        "NaN",
        Equal,
    );
    check(
        "Infinity", "Infinity", "Infinity", "Infinity", "1/3", 10, Floor, "Infinity", "Infinity",
        Equal,
    );
    check(
        "1.0",
        "0x1.0",
        "Infinity",
        "Infinity",
        "-1/3",
        10,
        Floor,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    check(
        "Infinity", "Infinity", "1.0", "0x1.0", "1/3", 10, Floor, "Infinity", "Infinity", Equal,
    );
    check(
        "1.0", "0x1.0", "0.0", "0x0.0", "1/3", 10, Floor, "1.0", "0x1.0", Equal,
    );
    check(
        "1.0", "0x1.0", "3.0", "0x3.0", "0", 10, Exact, "1.0", "0x1.0", Equal,
    );
    check(
        "0.0", "0x0.0", "3.0", "0x3.0", "1/3", 10, Exact, "1.0", "0x1.0", Equal,
    );
    check(
        "-1.0", "-0x1.0", "3.0", "0x3.0", "1/3", 10, Exact, "0.0", "0x0.0", Equal,
    );
    check(
        "1.0", "0x1.0", "3.0", "0x3.0", "-1/3", 10, Exact, "0.0", "0x0.0", Equal,
    );
    check(
        "1.0", "0x1.0", "2.0", "0x2.0", "3/4", 3, Exact, "2.5", "0x2.8", Equal,
    );
    check(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        "2.7182818284590451",
        "0x2.b7e151628aed2",
        "1/3",
        1,
        Nearest,
        "4.0",
        "0x4.0",
        Less,
    );
    check(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        "2.7182818284590451",
        "0x2.b7e151628aed2",
        "1/3",
        10,
        Floor,
        "4.047",
        "0x4.0c",
        Less,
    );
    check(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        "2.7182818284590451",
        "0x2.b7e151628aed2",
        "1/3",
        10,
        Ceiling,
        "4.0547",
        "0x4.0e",
        Greater,
    );
    check(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        "2.7182818284590451",
        "0x2.b7e151628aed2",
        "1/3",
        100,
        Nearest,
        "4.04768659640947481",
        "0x4.0c35305409476",
        Equal,
    );
    check(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        "2.7182818284590451",
        "0x2.b7e151628aed2",
        "-22/7",
        1,
        Nearest,
        "-4.0",
        "-0x4.0",
        Greater,
    );
    check(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        "2.7182818284590451",
        "0x2.b7e151628aed2",
        "-22/7",
        10,
        Floor,
        "-5.406",
        "-0x5.68",
        Less,
    );
    check(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        "2.7182818284590451",
        "0x2.b7e151628aed2",
        "-22/7",
        10,
        Ceiling,
        "-5.3984",
        "-0x5.66",
        Greater,
    );
    check(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        "2.7182818284590451",
        "0x2.b7e151628aed2",
        "-22/7",
        100,
        Nearest,
        "-5.4015788072814914550739168979418",
        "-0x5.66cdde640a6ad249249249248",
        Greater,
    );
    check(
        "2.4e30102999566398119521",
        "0x1.0E+25000000000000000000",
        "1.0",
        "0x1.0",
        "1/3",
        1,
        Nearest,
        "2.4e30102999566398119521",
        "0x1.0E+25000000000000000000",
        Less,
    );
    check(
        "2.4e30102999566398119521",
        "0x1.0E+25000000000000000000",
        "1.0",
        "0x1.0",
        "1/3",
        10,
        Floor,
        "2.4e30102999566398119521",
        "0x1.0E+25000000000000000000",
        Less,
    );
    check(
        "2.4e30102999566398119521",
        "0x1.0E+25000000000000000000",
        "1.0",
        "0x1.0",
        "1/3",
        10,
        Ceiling,
        "2.3699e30102999566398119521",
        "0x1.008E+25000000000000000000",
        Greater,
    );
    check(
        "2.4e30102999566398119521",
        "0x1.0E+25000000000000000000",
        "1.0",
        "0x1.0",
        "1/3",
        100,
        Nearest,
        "2.4e30102999566398119521",
        "0x1.0E+25000000000000000000",
        Less,
    );
    check(
        "4.2e-30102999566398119522",
        "0x1.0E-25000000000000000000",
        "1.0",
        "0x1.0",
        "1/3",
        1,
        Nearest,
        "0.25",
        "0x0.4",
        Less,
    );
    check(
        "4.2e-30102999566398119522",
        "0x1.0E-25000000000000000000",
        "1.0",
        "0x1.0",
        "1/3",
        10,
        Floor,
        "0.3330",
        "0x0.554",
        Less,
    );
    check(
        "4.2e-30102999566398119522",
        "0x1.0E-25000000000000000000",
        "1.0",
        "0x1.0",
        "1/3",
        10,
        Ceiling,
        "0.33350",
        "0x0.556",
        Greater,
    );
    check(
        "4.2e-30102999566398119522",
        "0x1.0E-25000000000000000000",
        "1.0",
        "0x1.0",
        "1/3",
        100,
        Nearest,
        "0.33333333333333333333333333333346",
        "0x0.55555555555555555555555558",
        Greater,
    );
    check(
        "1.0",
        "0x1.0",
        "2.4e30102999566398119521",
        "0x1.0E+25000000000000000000",
        "1/3",
        1,
        Nearest,
        "5.9e30102999566398119520",
        "0x4.0E+24999999999999999999",
        Less,
    );
    check(
        "1.0",
        "0x1.0",
        "2.4e30102999566398119521",
        "0x1.0E+25000000000000000000",
        "1/3",
        10,
        Floor,
        "7.877e30102999566398119520",
        "0x5.54E+24999999999999999999",
        Less,
    );
    check(
        "1.0",
        "0x1.0",
        "2.4e30102999566398119521",
        "0x1.0E+25000000000000000000",
        "1/3",
        10,
        Ceiling,
        "7.8882e30102999566398119520",
        "0x5.56E+24999999999999999999",
        Greater,
    );
    check(
        "1.0",
        "0x1.0",
        "2.4e30102999566398119521",
        "0x1.0E+25000000000000000000",
        "1/3",
        100,
        Nearest,
        "7.8843921656262388210459797389682e30102999566398119520",
        "0x5.5555555555555555555555558E+24999999999999999999",
        Greater,
    );
    check(
        "1.0",
        "0x1.0",
        "4.2e-30102999566398119522",
        "0x1.0E-25000000000000000000",
        "1/3",
        1,
        Nearest,
        "1.0",
        "0x1.0",
        Less,
    );
    check(
        "1.0",
        "0x1.0",
        "4.2e-30102999566398119522",
        "0x1.0E-25000000000000000000",
        "1/3",
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
        "1/3",
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
        "1/3",
        100,
        Nearest,
        "1.0",
        "0x1.0",
        Less,
    );
    check(
        "0.33333333326",
        "0x0.55555555",
        "1.0",
        "0x1.0",
        "-1/3",
        1,
        Nearest,
        "-5.8e-11",
        "-0x4.0E-9",
        Greater,
    );
    check(
        "0.33333333326",
        "0x0.55555555",
        "1.0",
        "0x1.0",
        "-1/3",
        10,
        Floor,
        "-7.7648e-11",
        "-0x5.56E-9",
        Less,
    );
    check(
        "0.33333333326",
        "0x0.55555555",
        "1.0",
        "0x1.0",
        "-1/3",
        10,
        Ceiling,
        "-7.753e-11",
        "-0x5.54E-9",
        Greater,
    );
    check(
        "0.33333333326",
        "0x0.55555555",
        "1.0",
        "0x1.0",
        "-1/3",
        100,
        Nearest,
        "-7.7610214551289876302083333333364e-11",
        "-0x5.5555555555555555555555558E-9",
        Less,
    );
    check(
        "-2.4e30102999566398119521",
        "-0x1.0E+25000000000000000000",
        "2.4e30102999566398119521",
        "0x1.0E+25000000000000000000",
        "1/3",
        1,
        Nearest,
        "-1.2e30102999566398119521",
        "-0x8.0E+24999999999999999999",
        Greater,
    );
    check(
        "-2.4e30102999566398119521",
        "-0x1.0E+25000000000000000000",
        "2.4e30102999566398119521",
        "0x1.0E+25000000000000000000",
        "1/3",
        10,
        Floor,
        "-1.5776e30102999566398119521",
        "-0xa.acE+24999999999999999999",
        Less,
    );
    check(
        "-2.4e30102999566398119521",
        "-0x1.0E+25000000000000000000",
        "2.4e30102999566398119521",
        "0x1.0E+25000000000000000000",
        "1/3",
        10,
        Ceiling,
        "-1.575e30102999566398119521",
        "-0xa.a8E+24999999999999999999",
        Greater,
    );
    check(
        "-2.4e30102999566398119521",
        "-0x1.0E+25000000000000000000",
        "2.4e30102999566398119521",
        "0x1.0E+25000000000000000000",
        "1/3",
        100,
        Nearest,
        "-1.5768784331252477642091959477936e30102999566398119521",
        "-0xa.aaaaaaaaaaaaaaaaaaaaaaabE+24999999999999999999",
        Less,
    );
}

#[test]
#[should_panic]
fn add_mul_rational_prec_round_fail_1() {
    BigExponentFloat::ONE.add_mul_rational_prec_round(
        BigExponentFloat::ONE,
        Rational::from_signeds(1, 3),
        0,
        Floor,
    );
}

#[test]
#[should_panic]
fn add_mul_rational_prec_round_fail_2() {
    BigExponentFloat::ONE.add_mul_rational_prec_round(
        BigExponentFloat::ONE,
        Rational::from_signeds(1, 3),
        10,
        Exact,
    );
}

#[test]
fn add_mul_rational_prec_round_properties() {
    let generator =
        big_exponent_float_big_exponent_float_rational_unsigned_rounding_mode_quintuple_gen_var_1;
    generator().test_properties(|(x, y, z, prec, rm)| {
        let r = add_mul_rational_prec_round_all(&x, &y, &z, prec, rm);
        if r.0.is_nan() || r.0.is_zero() || r.0.is_infinite() {
            assert_eq!(r.1, Equal);
        } else {
            assert!(r.0.get_prec().unwrap() <= prec);
        }
        // The kernels agree with the kernel-free implementation, at any exponents.
        assert!(same_pair(
            &add_mul_rational_prec_round_generic(&x, &y, &z, false, prec, rm),
            &r
        ));
        // Negating z switches between addition and subtraction.
        assert!(same_pair(
            &x.sub_mul_rational_prec_round_ref_ref_val(&y, -&z, prec, rm),
            &r
        ));
        // A dyadic z gives the operation on three `BigExponentFloat`s.
        if let Ok(zb) = BigExponentFloat::try_from(&z) {
            assert!(same_pair(
                &x.add_mul_prec_round_ref_ref_ref(&y, &zb, prec, rm),
                &r
            ));
        }
        if [&x, &y]
            .iter()
            .all(|v| big_exponent_float_exponent_is_small(v) && v.is_finite())
            && r.0.is_finite()
        {
            let e = Rational::exact_from(&x) + Rational::exact_from(&y) * &z;
            assert_eq!(Rational::exact_from(&r.0).cmp(&e), r.1);
        }
    });
}

#[test]
fn add_mul_rational_prec_round_float_agreement_properties() {
    let generator = float_float_rational_unsigned_rounding_mode_quintuple_gen_var_1;
    generator().test_properties(|(x, y, z, prec, rm)| {
        let (f, fo) = x.add_mul_rational_prec_round_ref_ref_ref(&y, &z, prec, rm);
        if float_result_comparable(&f, fo) {
            let r = add_mul_rational_prec_round_all(&to_bef(&x), &to_bef(&y), &z, prec, rm);
            assert!(same(&r.0, &to_bef(&f)));
            assert_eq!(r.1, fo);
        }
    });
    let generator = float_float_rational_unsigned_rounding_mode_quintuple_gen_var_2;
    generator().test_properties(|(x, y, z, prec, rm)| {
        let (f, fo) = x.add_mul_rational_prec_round_ref_ref_ref(&y, &z, prec, rm);
        if float_result_comparable(&f, fo) {
            let r = add_mul_rational_prec_round_all(&to_bef(&x), &to_bef(&y), &z, prec, rm);
            assert!(same(&r.0, &to_bef(&f)));
            assert_eq!(r.1, fo);
        }
    });
}
