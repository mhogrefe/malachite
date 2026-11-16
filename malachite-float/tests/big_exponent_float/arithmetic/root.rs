// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{Abs, Pow};
use malachite_base::num::basic::traits::{One, Two};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_float::big_exponent_float::ComparableBigExponentFloatRef;
use malachite_float::big_exponent_float::arithmetic::root::root_prec_round_generic;
use malachite_float::test_util::big_exponent_float::{
    float_to_big_exponent_float, parse_big_exponent_float_hex,
};
use malachite_float::test_util::generators::{
    big_exponent_float_signed_unsigned_rounding_mode_quadruple_gen_var_1,
    big_exponent_float_unsigned_pair_gen_var_2,
    big_exponent_float_unsigned_rounding_mode_triple_gen_var_1,
    big_exponent_float_unsigned_unsigned_rounding_mode_quadruple_gen_var_1,
    float_signed_unsigned_rounding_mode_quadruple_gen_var_13,
    float_unsigned_unsigned_rounding_mode_quadruple_gen_var_13,
    rational_signed_unsigned_rounding_mode_quadruple_gen_var_2,
    rational_unsigned_unsigned_rounding_mode_quadruple_gen_var_3,
};
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

// Whether `x` is finite and nonzero.
const fn finite_nonzero(x: &BigExponentFloat) -> bool {
    x.is_finite() && !x.is_zero()
}

fn root_u_prec_round_all(
    x: &BigExponentFloat,
    k: u64,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = x.clone().root_u_prec_round(k, prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(&x.root_u_prec_round_ref(k, prec, rm), &r));
    let mut z = x.clone();
    let o = z.root_u_prec_round_assign(k, prec, rm);
    assert!(same_pair(&(z, o), &r));
    if rm == Nearest {
        assert!(same_pair(&x.clone().root_u_prec(k, prec), &r));
        assert!(same_pair(&x.root_u_prec_ref(k, prec), &r));
        let mut z = x.clone();
        let o = z.root_u_prec_assign(k, prec);
        assert!(same_pair(&(z, o), &r));
    }
    if let Ok(k) = i64::try_from(k) {
        assert!(same_pair(&root_s_prec_round_all(x, k, prec, rm), &r));
    }
    r
}

fn root_s_prec_round_all(
    x: &BigExponentFloat,
    k: i64,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = x.clone().root_s_prec_round(k, prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(&x.root_s_prec_round_ref(k, prec, rm), &r));
    let mut z = x.clone();
    let o = z.root_s_prec_round_assign(k, prec, rm);
    assert!(same_pair(&(z, o), &r));
    if rm == Nearest {
        assert!(same_pair(&x.clone().root_s_prec(k, prec), &r));
        assert!(same_pair(&x.root_s_prec_ref(k, prec), &r));
        let mut z = x.clone();
        let o = z.root_s_prec_assign(k, prec);
        assert!(same_pair(&(z, o), &r));
    }
    r
}

fn check_u(
    x: &str,
    x_hex: &str,
    k: u64,
    prec: u64,
    rm: RoundingMode,
    out: &str,
    out_hex: &str,
    o: Ordering,
) {
    assert_eq!(parse_big_exponent_float_hex(x_hex).to_string(), x);
    let x = &parse_big_exponent_float_hex(x_hex);
    assert_eq!(parse_big_exponent_float_hex(out_hex).to_string(), out);
    let out = &parse_big_exponent_float_hex(out_hex);
    let r = root_u_prec_round_all(x, k, prec, rm);
    assert!(
        same(&r.0, out),
        "{x:?} {k} {prec} {rm}: {:?} != {out:?}",
        r.0
    );
    assert_eq!(r.1, o);
    assert!(same_pair(
        &root_prec_round_generic(x, i128::from(k), prec, rm),
        &r
    ));
}

fn check_s(
    x: &str,
    x_hex: &str,
    k: i64,
    prec: u64,
    rm: RoundingMode,
    out: &str,
    out_hex: &str,
    o: Ordering,
) {
    assert_eq!(parse_big_exponent_float_hex(x_hex).to_string(), x);
    let x = &parse_big_exponent_float_hex(x_hex);
    assert_eq!(parse_big_exponent_float_hex(out_hex).to_string(), out);
    let out = &parse_big_exponent_float_hex(out_hex);
    let r = root_s_prec_round_all(x, k, prec, rm);
    assert!(
        same(&r.0, out),
        "{x:?} {k} {prec} {rm}: {:?} != {out:?}",
        r.0
    );
    assert_eq!(r.1, o);
    assert!(same_pair(
        &root_prec_round_generic(x, i128::from(k), prec, rm),
        &r
    ));
}

fn check_u_pi(
    x: &str,
    x_hex: &str,
    prec: u64,
    rm: RoundingMode,
    out: &str,
    out_hex: &str,
    o: Ordering,
) {
    check_u(x, x_hex, 4, prec, rm, out, out_hex, o);
}

fn check_s_pi(
    x: &str,
    x_hex: &str,
    prec: u64,
    rm: RoundingMode,
    out: &str,
    out_hex: &str,
    o: Ordering,
) {
    check_s(x, x_hex, -4, prec, rm, out, out_hex, o);
}

#[test]
fn test_root_u_prec_round() {
    check_u("1.0", "0x1.0", 0, 10, Floor, "NaN", "NaN", Equal);
    check_u("NaN", "NaN", 3, 10, Floor, "NaN", "NaN", Equal);
    check_u(
        "Infinity", "Infinity", 4, 10, Floor, "Infinity", "Infinity", Equal,
    );
    check_u(
        "-Infinity",
        "-Infinity",
        5,
        10,
        Floor,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    check_u("-Infinity", "-Infinity", 4, 10, Floor, "NaN", "NaN", Equal);
    check_u("0.0", "0x0.0", 4, 10, Floor, "0.0", "0x0.0", Equal);
    check_u("-16.0", "-0x1.0E+1", 4, 10, Floor, "NaN", "NaN", Equal);
    check_u("81.00", "0x51.0", 4, 2, Exact, "3.0", "0x3.0", Equal);
    check_u("-243.0", "-0xf3.0", 5, 2, Exact, "-3.0", "-0x3.0", Equal);
    check_u("3.0", "0x3.0", 1, 1, Floor, "2.0", "0x2.0", Less);
    // 2^(5N)^(1/5) = 2^N, beyond the range of a `Float`; 2^(5N + 2)^(1/5) = 2^N * 2^(2/5) rounds
    // down to 2^N and up to 2^(N + 1) at 1 bit.
    check_u(
        "7.4e150514997831990597606",
        "0x1.0E+125000000000000000000",
        5,
        1,
        Exact,
        "2.4e30102999566398119521",
        "0x1.0E+25000000000000000000",
        Equal,
    );
    check_u(
        "1.4e-150514997831990597607",
        "0x1.0E-125000000000000000000",
        5,
        1,
        Exact,
        "4.2e-30102999566398119522",
        "0x1.0E-25000000000000000000",
        Equal,
    );
    check_u(
        "3.0e150514997831990597607",
        "0x4.0E+125000000000000000000",
        5,
        1,
        Floor,
        "2.4e30102999566398119521",
        "0x1.0E+25000000000000000000",
        Less,
    );
    check_u(
        "3.0e150514997831990597607",
        "0x4.0E+125000000000000000000",
        5,
        1,
        Ceiling,
        "4.7e30102999566398119521",
        "0x2.0E+25000000000000000000",
        Greater,
    );
    // An order beyond `Float`'s exponent range, with an exponent that cannot be scaled into it, so
    // that the generic algorithm is used: for k = 2^63 + 1, (2^(k + 2^62))^(1/k) = 2^(1 + 2^62/k),
    // and 2^62/k = 1/2 - 1/(2^64 + 2), so the root is sqrt(8) * (1 - O(2^-64)). That rounds like
    // sqrt(8), which is irrational and not within 2^-60 of a 10-bit rounding breakpoint.
    let k = (1u64 << 63) + 1;
    let o = BigExponentFloat::from(8).sqrt_prec_round(10, Floor).1;
    assert_eq!(o, Less);
    check_u(
        "3.2e4164767466392517849",
        "0x2.0E+3458764513820540928",
        k,
        10,
        Floor,
        "2.828",
        "0x2.d4",
        Less,
    );
    check_u(
        "3.2e4164767466392517849",
        "0x2.0E+3458764513820540928",
        k,
        10,
        Ceiling,
        "2.8320",
        "0x2.d5",
        Greater,
    );
    check_u(
        "-3.2e4164767466392517849",
        "-0x2.0E+3458764513820540928",
        k,
        10,
        Ceiling,
        "-2.828",
        "-0x2.d4",
        Greater,
    );
    // ... and an exact root at such an order: (2^(2k))^(1/k) = 4.
    check_u(
        "7.6e5553023288523357132",
        "0x4.0E+4611686018427387904",
        k,
        1,
        Exact,
        "4.0",
        "0x4.0",
        Equal,
    );
    check_u_pi(
        "3.1415926535897931",
        "0x3.243f6a8885a3",
        5,
        Floor,
        "1.31",
        "0x1.5",
        Less,
    );
    check_u_pi(
        "3.1415926535897931",
        "0x3.243f6a8885a3",
        5,
        Ceiling,
        "1.38",
        "0x1.6",
        Greater,
    );
    check_u_pi(
        "3.1415926535897931",
        "0x3.243f6a8885a3",
        20,
        Nearest,
        "1.3313351",
        "0x1.54d26",
        Less,
    );
}

#[test]
fn test_root_s_prec_round() {
    check_s("1.0", "0x1.0", 0, 10, Floor, "NaN", "NaN", Equal);
    check_s("NaN", "NaN", -3, 10, Floor, "NaN", "NaN", Equal);
    check_s("Infinity", "Infinity", -4, 10, Floor, "0.0", "0x0.0", Equal);
    check_s(
        "-Infinity",
        "-Infinity",
        -5,
        10,
        Floor,
        "0.0",
        "0x0.0",
        Equal,
    );
    check_s("-Infinity", "-Infinity", -4, 10, Floor, "NaN", "NaN", Equal);
    // Without a negative zero, 0^(1/k) for negative odd k could be either infinity.
    check_s("0.0", "0x0.0", -4, 10, Floor, "Infinity", "Infinity", Equal);
    check_s("0.0", "0x0.0", -3, 10, Floor, "NaN", "NaN", Equal);
    check_s("0.0", "0x0.0", -1, 10, Floor, "NaN", "NaN", Equal);
    check_s("0.0", "0x0.0", -2, 10, Floor, "Infinity", "Infinity", Equal);
    // 81^(-1/4) = 1/3.
    let o = BigExponentFloat::from(3).reciprocal_prec_round(2, Floor).1;
    check_s("81.00", "0x51.0", -4, 2, Floor, "0.25", "0x0.4", o);
    check_s("0.031", "0x0.08", -5, 2, Exact, "2.0", "0x2.0", Equal);
    check_s("-0.031", "-0x0.08", -5, 2, Exact, "-2.0", "-0x2.0", Equal);
    check_s(
        "7.4e150514997831990597606",
        "0x1.0E+125000000000000000000",
        -5,
        1,
        Exact,
        "4.2e-30102999566398119522",
        "0x1.0E-25000000000000000000",
        Equal,
    );
    check_s(
        "3.0e150514997831990597607",
        "0x4.0E+125000000000000000000",
        -5,
        1,
        Floor,
        "2.1e-30102999566398119522",
        "0x8.0E-25000000000000000001",
        Less,
    );
    let k = i64::MIN;
    // (2^(2^64))^(1/-2^63) = 1/4.
    check_s(
        "1.9e5553023288523357132",
        "0x1.0E+4611686018427387904",
        k,
        1,
        Exact,
        "0.25",
        "0x0.4",
        Equal,
    );
    // (2^(2^63 + 2^62))^(1/-2^63) = 2^(-3/2) = sqrt(1/8).
    let o = BigExponentFloat::from(0.125).sqrt_prec_round(10, Nearest).1;
    check_s(
        "1.6e4164767466392517849",
        "0x1.0E+3458764513820540928",
        k,
        10,
        Nearest,
        "0.3535",
        "0x0.5a8",
        o,
    );
    check_s_pi(
        "3.1415926535897931",
        "0x3.243f6a8885a3",
        5,
        Floor,
        "0.75",
        "0x0.c",
        Less,
    );
    check_s_pi(
        "3.1415926535897931",
        "0x3.243f6a8885a3",
        5,
        Ceiling,
        "0.781",
        "0x0.c8",
        Greater,
    );
    check_s_pi(
        "3.1415926535897931",
        "0x3.243f6a8885a3",
        20,
        Nearest,
        "0.7511253",
        "0x0.c049c",
        Less,
    );
}

// Orders too large to scale x by a multiple of the order into `Float`'s exponent range by the
// remainder r alone, so that r - |k| is used, or the generic algorithm; the results agree with
// `Float`'s, which needs no scaling for these x.
#[test]
fn test_root_large_order_float_agreement() {
    let xs = [0.1, 3.0, 1.0e-300, 1.0e300, 0.75, -0.1, -3.0];
    let ks = [(1u64 << 31) + 1, (1 << 40) + 1, (1 << 62) + 1, u64::MAX];
    for x in xs {
        for k in ks {
            for prec in [1, 10, 100] {
                for rm in [Floor, Ceiling, Down, Up, Nearest] {
                    let (f, fo) = Float::from(x).root_u_prec_round(k, prec, rm);
                    let r = root_u_prec_round_all(&BigExponentFloat::from(x), k, prec, rm);
                    assert!(
                        same(&r.0, &float_to_big_exponent_float(&f)),
                        "{x} {k} {prec} {rm}"
                    );
                    assert_eq!(r.1, fo);
                    let k = -i64::try_from(k >> 1).unwrap();
                    let (f, fo) = Float::from(x).root_s_prec_round(k, prec, rm);
                    let r = root_s_prec_round_all(&BigExponentFloat::from(x), k, prec, rm);
                    assert!(
                        same(&r.0, &float_to_big_exponent_float(&f)),
                        "{x} {k} {prec} {rm}"
                    );
                    assert_eq!(r.1, fo);
                    assert!(same_pair(
                        &root_prec_round_generic(
                            &BigExponentFloat::from(x),
                            i128::from(k),
                            prec,
                            rm
                        ),
                        &r
                    ));
                }
            }
        }
    }
}

#[test]
#[should_panic]
fn root_u_prec_round_fail_1() {
    BigExponentFloat::ONE.root_u_prec_round(3, 0, Floor);
}

#[test]
#[should_panic]
fn root_u_prec_round_fail_2() {
    BigExponentFloat::TWO.root_u_prec_round(3, 10, Exact);
}

#[test]
#[should_panic]
fn root_s_prec_round_fail() {
    BigExponentFloat::TWO.root_s_prec_round(-3, 10, Exact);
}

// Checks the ordering of the rounded root r of order k of x: r compares with the exact root as
// r^|k| compares with x, or, for negative k and positive x, as r^|k| * x compares with 1.
fn check_root_ordering(x: &BigExponentFloat, k: i64, r: &(BigExponentFloat, Ordering)) {
    if !finite_nonzero(x) || !finite_nonzero(&r.0) || k.unsigned_abs() > 64 || k == 0 {
        return;
    }
    let power = (&r.0).pow(k.unsigned_abs());
    if k > 0 {
        // Odd powers and powers of positive values are increasing.
        assert_eq!(power.partial_cmp(x), Some(r.1));
    } else if x.is_sign_positive() {
        // x^(1/k) = 1/x^(1/|k|), and r is too large iff r^|k| * x > 1.
        assert_eq!((power * x).partial_cmp(&BigExponentFloat::ONE), Some(r.1));
    }
}

fn root_properties_helper(x: &BigExponentFloat, k: i64, prec: u64, rm: RoundingMode) {
    let r = root_s_prec_round_all(x, k, prec, rm);
    // The kernels agree with the kernel-free implementation, at any exponents.
    assert!(same_pair(
        &root_prec_round_generic(x, i128::from(k), prec, rm),
        &r
    ));
    if r.0.is_nan() || r.0.is_zero() || r.0.is_infinite() {
        assert_eq!(r.1, Equal);
    } else {
        assert!(r.0.get_prec().unwrap() <= prec);
    }
    if let Ok(shift) = u32::try_from(k.unsigned_abs())
        && shift <= 1000
    {
        // Multiplying x by 2^|k| multiplies the root by 2^sign(k).
        let (s, o) = (x << shift).root_s_prec_round(k, prec, rm);
        assert!(same(&s, &(&r.0 << k.signum())));
        assert_eq!(o, r.1);
    }
    if k % 2 != 0 {
        // Negating x negates the root.
        let (s, o) = (-x).root_s_prec_round(k, prec, -rm);
        assert!(same(&s, &-&r.0));
        assert_eq!(o, r.1.reverse());
    }
    check_root_ordering(x, k, &r);
}

#[test]
fn root_u_prec_round_properties() {
    big_exponent_float_unsigned_unsigned_rounding_mode_quadruple_gen_var_1().test_properties(
        |(x, k, prec, rm)| {
            let r = root_u_prec_round_all(&x, k, prec, rm);
            assert!(same_pair(
                &root_prec_round_generic(&x, i128::from(k), prec, rm),
                &r
            ));
            if let Ok(k) = i64::try_from(k) {
                root_properties_helper(&x, k, prec, rm);
            }
            match k {
                2 => assert!(same_pair(&x.sqrt_prec_round_ref(prec, rm), &r)),
                3 => assert!(same_pair(&x.cbrt_prec_round_ref(prec, rm), &r)),
                _ => {}
            }
        },
    );
}

#[test]
fn root_s_prec_round_properties() {
    big_exponent_float_signed_unsigned_rounding_mode_quadruple_gen_var_1().test_properties(
        |(x, k, prec, rm)| {
            root_properties_helper(&x, k, prec, rm);
            let r = root_s_prec_round_all(&x, k, prec, rm);
            match k {
                -1 => assert!(same_pair(&x.reciprocal_prec_round_ref(prec, rm), &r)),
                -2 => assert!(same_pair(&x.reciprocal_sqrt_prec_round_ref(prec, rm), &r)),
                _ => {}
            }
        },
    );
}

#[test]
fn root_prec_round_properties_2() {
    big_exponent_float_unsigned_pair_gen_var_2().test_properties(|(x, k)| {
        if finite_nonzero(&x) && (1..=16).contains(&k) {
            // The kth root of an exact kth power is exact.
            let prec = x.get_prec().unwrap();
            let y = (&x).pow(k);
            let expected = if k % 2 == 0 { (&x).abs() } else { x.clone() };
            assert!(same_pair(
                &y.root_u_prec_round(k, prec, Exact),
                &(expected, Equal)
            ));
        }
    });
}

#[test]
fn root_prec_round_properties_3() {
    big_exponent_float_unsigned_rounding_mode_triple_gen_var_1().test_properties(
        |(x, prec, rm)| {
            // The first root rounds.
            assert!(same_pair(
                &x.root_u_prec_round_ref(1, prec, rm),
                &x.round_to_prec_round_ref(prec, rm)
            ));
        },
    );
}

#[test]
fn root_prec_round_float_agreement_properties() {
    float_unsigned_unsigned_rounding_mode_quadruple_gen_var_13().test_properties(
        |(x, k, prec, rm)| {
            let (f, fo) = x.root_u_prec_round_ref(k, prec, rm);
            if float_result_comparable(&f, fo) {
                let r = root_u_prec_round_all(&float_to_big_exponent_float(&x), k, prec, rm);
                assert!(same(&r.0, &float_to_big_exponent_float(&f)));
                assert_eq!(r.1, fo);
            }
        },
    );
    float_signed_unsigned_rounding_mode_quadruple_gen_var_13().test_properties(
        |(x, k, prec, rm)| {
            let (f, fo) = x.root_s_prec_round_ref(k, prec, rm);
            // `Float` has a negative zero, so it gives an infinity for 0^(1/k) with negative odd k.
            if float_result_comparable(&f, fo) && !(x.is_zero() && k < 0 && k % 2 != 0) {
                let r = root_s_prec_round_all(&float_to_big_exponent_float(&x), k, prec, rm);
                assert!(same(&r.0, &float_to_big_exponent_float(&f)));
                assert_eq!(r.1, fo);
            }
        },
    );
}

fn root_u_rational_prec_round_all(
    x: &Rational,
    k: u64,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = BigExponentFloat::root_u_rational_prec_round(x.clone(), k, prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(
        &BigExponentFloat::root_u_rational_prec_round_ref(x, k, prec, rm),
        &r
    ));
    if rm == Nearest {
        assert!(same_pair(
            &BigExponentFloat::root_u_rational_prec(x.clone(), k, prec),
            &r
        ));
        assert!(same_pair(
            &BigExponentFloat::root_u_rational_prec_ref(x, k, prec),
            &r
        ));
    }
    r
}

#[test]
fn test_root_u_rational_prec_round() {
    let test =
        |x: &str, k: u64, prec: u64, rm: RoundingMode, out: &str, out_hex: &str, o: Ordering| {
            assert_eq!(parse_big_exponent_float_hex(out_hex).to_string(), out);
            let r = root_u_rational_prec_round_all(&Rational::from_str(x).unwrap(), k, prec, rm);
            assert!(
                same(&r.0, &parse_big_exponent_float_hex(out_hex)),
                "{x} {prec} {rm}: {:?}",
                r.0
            );
            assert_eq!(r.1, o, "{x} {prec} {rm}");
        };
    test("3/5", 0, 10, Exact, "NaN", "NaN", Equal);
    test("0", 4, 10, Exact, "0.0", "0x0.0", Equal);
    test("-3/5", 4, 10, Exact, "NaN", "NaN", Equal);
    test("-27/8", 3, 2, Exact, "-1.5", "-0x1.8", Equal);
    test("81/16", 4, 2, Exact, "1.5", "0x1.8", Equal);
    test("3/5", 4, 1, Nearest, "1.0", "0x1.0", Greater);
    test("3/5", 4, 10, Floor, "0.87988", "0x0.e14", Less);
    test("3/5", 4, 10, Ceiling, "0.8809", "0x0.e18", Greater);
    test(
        "3/5",
        4,
        100,
        Nearest,
        "0.8801117367933933972710824055660",
        "0x0.e14f00b65a7768edc51e3d8cc",
        Greater,
    );
    test("22/7", 5, 1, Nearest, "1.0", "0x1.0", Less);
    test("22/7", 5, 10, Floor, "1.2559", "0x1.418", Less);
    test("22/7", 5, 10, Ceiling, "1.258", "0x1.42", Greater);
    test(
        "22/7",
        5,
        100,
        Nearest,
        "1.2573753098024454738217199273498",
        "0x1.41e3592a663d36701d8541336",
        Greater,
    );
    test("-22/7", 5, 1, Nearest, "-1.0", "-0x1.0", Greater);
    test("-22/7", 5, 10, Floor, "-1.258", "-0x1.42", Less);
    test("-22/7", 5, 10, Ceiling, "-1.2559", "-0x1.418", Greater);
    test(
        "-22/7",
        5,
        100,
        Nearest,
        "-1.2573753098024454738217199273498",
        "-0x1.41e3592a663d36701d8541336",
        Less,
    );
    test(
        "1/100000000000000000000000",
        7,
        1,
        Nearest,
        "0.00049",
        "0x0.002",
        Less,
    );
    test(
        "1/100000000000000000000000",
        7,
        10,
        Floor,
        "0.00051785",
        "0x0.0021f",
        Less,
    );
    test(
        "1/100000000000000000000000",
        7,
        10,
        Ceiling,
        "0.000519",
        "0x0.0022",
        Greater,
    );
    test(
        "1/100000000000000000000000",
        7,
        100,
        Nearest,
        "0.00051794746792312111347551746779597",
        "0x0.0021f1b76f8f00f67b8752c98acc",
        Less,
    );
    test("3/5", 1, 1, Nearest, "0.50", "0x0.8", Less);
    test("3/5", 1, 10, Floor, "0.5996", "0x0.998", Less);
    test("3/5", 1, 10, Ceiling, "0.60059", "0x0.99c", Greater);
    test(
        "3/5",
        1,
        100,
        Nearest,
        "0.6000000000000000000000000000003",
        "0x0.999999999999999999999999a",
        Greater,
    );
    test("3/5", 1000000, 1, Nearest, "1.0", "0x1.0", Greater);
    test("3/5", 1000000, 10, Floor, "0.99902", "0x0.ffc", Less);
    test("3/5", 1000000, 10, Ceiling, "1.0", "0x1.0", Greater);
    test(
        "3/5",
        1000000,
        100,
        Nearest,
        "0.99999948917450670539604870776903",
        "0x0.fffff76e056e2bf4019a8a763",
        Less,
    );
}

#[test]
#[should_panic]
fn root_u_rational_prec_round_fail_1() {
    BigExponentFloat::root_u_rational_prec_round(Rational::from_unsigneds(3u8, 5), 5, 0, Floor);
}

#[test]
#[should_panic]
fn root_u_rational_prec_round_fail_2() {
    BigExponentFloat::root_u_rational_prec_round(Rational::from_unsigneds(3u8, 5), 5, 10, Exact);
}

#[test]
fn root_u_rational_prec_round_properties() {
    rational_unsigned_unsigned_rounding_mode_quadruple_gen_var_3().test_properties(
        |(x, k, prec, rm)| {
            let r = root_u_rational_prec_round_all(&x, k, prec, rm);
            if r.0.is_nan() || r.0.is_zero() || r.0.is_infinite() {
                assert_eq!(r.1, Equal);
            } else {
                assert!(r.0.get_prec().unwrap() <= prec);
            }
            // Multiplying x by 2^(1000k) multiplies the root by 2^(1000 sign(k)).
            if r.0.is_finite() && !r.0.is_zero() && k <= 1000 {
                let s = 1000 * i64::exact_from(k);
                let (y, o) = root_u_rational_prec_round_all(&(&x << s), k, prec, rm);
                assert!(same(&y, &(&r.0 << 1000)));
                assert_eq!(o, r.1);
            }
            // A dyadic x gives the root of the equal `BigExponentFloat`.
            if let Ok(x_bef) = BigExponentFloat::try_from(&x) {
                assert!(same_pair(&x_bef.root_u_prec_round(k, prec, rm), &r));
            }
            // `Float`'s function agrees wherever its result neither overflows nor underflows (at 0,
            // `Float` has signed zeros and infinities).
            if x != 0u32 {
                let (f, fo) = Float::root_u_rational_prec_round_ref(&x, k, prec, rm);
                if float_result_comparable(&f, fo) {
                    assert!(same(&r.0, &float_to_big_exponent_float(&f)));
                    assert_eq!(r.1, fo);
                }
            }
        },
    );
}

fn root_s_rational_prec_round_all(
    x: &Rational,
    k: i64,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = BigExponentFloat::root_s_rational_prec_round(x.clone(), k, prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(
        &BigExponentFloat::root_s_rational_prec_round_ref(x, k, prec, rm),
        &r
    ));
    if rm == Nearest {
        assert!(same_pair(
            &BigExponentFloat::root_s_rational_prec(x.clone(), k, prec),
            &r
        ));
        assert!(same_pair(
            &BigExponentFloat::root_s_rational_prec_ref(x, k, prec),
            &r
        ));
    }
    r
}

#[test]
fn test_root_s_rational_prec_round() {
    let test =
        |x: &str, k: i64, prec: u64, rm: RoundingMode, out: &str, out_hex: &str, o: Ordering| {
            assert_eq!(parse_big_exponent_float_hex(out_hex).to_string(), out);
            let r = root_s_rational_prec_round_all(&Rational::from_str(x).unwrap(), k, prec, rm);
            assert!(
                same(&r.0, &parse_big_exponent_float_hex(out_hex)),
                "{x} {prec} {rm}: {:?}",
                r.0
            );
            assert_eq!(r.1, o, "{x} {prec} {rm}");
        };
    test("3/5", 0, 10, Exact, "NaN", "NaN", Equal);
    test("0", 3, 10, Exact, "0.0", "0x0.0", Equal);
    test("0", -2, 10, Exact, "Infinity", "Infinity", Equal);
    test("0", -3, 10, Exact, "NaN", "NaN", Equal);
    test("-3/5", -4, 10, Exact, "NaN", "NaN", Equal);
    test("16/81", -4, 2, Exact, "1.5", "0x1.8", Equal);
    test("3/5", -3, 1, Nearest, "1.0", "0x1.0", Less);
    test("3/5", -3, 10, Floor, "1.1855", "0x1.2f8", Less);
    test("3/5", -3, 10, Ceiling, "1.19", "0x1.3", Greater);
    test(
        "3/5",
        -3,
        100,
        Nearest,
        "1.1856311014966876020020515407464",
        "0x1.2f8585160c79901b6927b11de",
        Greater,
    );
    test("22/7", -5, 1, Nearest, "1.0", "0x1.0", Greater);
    test("22/7", -5, 10, Floor, "0.7949", "0x0.cb8", Less);
    test("22/7", -5, 10, Ceiling, "0.79590", "0x0.cbc", Greater);
    test(
        "22/7",
        -5,
        100,
        Nearest,
        "0.7953074887060702616288572906089",
        "0x0.cb99458641a76fc7a6e7edcda",
        Greater,
    );
    test("-22/7", -5, 1, Nearest, "-1.0", "-0x1.0", Less);
    test("-22/7", -5, 10, Floor, "-0.79590", "-0x0.cbc", Less);
    test("-22/7", -5, 10, Ceiling, "-0.7949", "-0x0.cb8", Greater);
    test(
        "-22/7",
        -5,
        100,
        Nearest,
        "-0.7953074887060702616288572906089",
        "-0x0.cb99458641a76fc7a6e7edcda",
        Less,
    );
    test(
        "1/100000000000000000000000",
        -7,
        1,
        Nearest,
        "2.0e3",
        "0x8.0E+2",
        Greater,
    );
    test(
        "1/100000000000000000000000",
        -7,
        10,
        Floor,
        "1930.0",
        "0x78a.0",
        Less,
    );
    test(
        "1/100000000000000000000000",
        -7,
        10,
        Ceiling,
        "1932.0",
        "0x78c.0",
        Greater,
    );
    test(
        "1/100000000000000000000000",
        -7,
        100,
        Nearest,
        "1930.6977288832501670070747998403",
        "0x78a.b29e5c2f08b8c70ea8412a8",
        Greater,
    );
    test("3/5", -1, 1, Nearest, "2.0", "0x2.0", Greater);
    test("3/5", -1, 10, Floor, "1.6660", "0x1.aa8", Less);
    test("3/5", -1, 10, Ceiling, "1.668", "0x1.ab", Greater);
    test(
        "3/5",
        -1,
        100,
        Nearest,
        "1.6666666666666666666666666666661",
        "0x1.aaaaaaaaaaaaaaaaaaaaaaaaa",
        Less,
    );
    test("3/5", 1, 1, Nearest, "0.50", "0x0.8", Less);
    test("3/5", 1, 10, Floor, "0.5996", "0x0.998", Less);
    test("3/5", 1, 10, Ceiling, "0.60059", "0x0.99c", Greater);
    test(
        "3/5",
        1,
        100,
        Nearest,
        "0.6000000000000000000000000000003",
        "0x0.999999999999999999999999a",
        Greater,
    );
    test("3/5", -1000000, 1, Nearest, "1.0", "0x1.0", Less);
    test("3/5", -1000000, 10, Floor, "1.0", "0x1.0", Less);
    test("3/5", -1000000, 10, Ceiling, "1.0020", "0x1.008", Greater);
    test(
        "3/5",
        -1000000,
        100,
        Nearest,
        "1.0000005108257542374218472113698",
        "0x1.00000891fadb46f55f92d587e",
        Less,
    );
}

#[test]
fn test_root_s_rational_prec_round_huge() {
    // (3 * 2^(-2^30 - 5))^(1/-1) = 2^(2^30 + 5) / 3, whose exponent is beyond `Float`'s range.
    let x = Rational::from(3u32) >> ((1u64 << 30) + 5);
    for rm in [Floor, Ceiling, Nearest] {
        let (y, o) =
            BigExponentFloat::from_rational_prec_round(Rational::from_unsigneds(1u8, 3), 20, rm);
        let r = root_s_rational_prec_round_all(&x, -1, 20, rm);
        assert!(same(&r.0, &(y << ((1u64 << 30) + 5))));
        assert_eq!(r.1, o);
        assert!(*r.0.get_exponent().unwrap() > Float::MAX_EXPONENT);
    }
}

#[test]
#[should_panic]
fn root_s_rational_prec_round_fail_1() {
    BigExponentFloat::root_s_rational_prec_round(Rational::from_unsigneds(3u8, 5), -5, 0, Floor);
}

#[test]
#[should_panic]
fn root_s_rational_prec_round_fail_2() {
    BigExponentFloat::root_s_rational_prec_round(Rational::from_unsigneds(3u8, 5), -5, 10, Exact);
}

#[test]
fn root_s_rational_prec_round_properties() {
    rational_signed_unsigned_rounding_mode_quadruple_gen_var_2().test_properties(
        |(x, k, prec, rm)| {
            let r = root_s_rational_prec_round_all(&x, k, prec, rm);
            if r.0.is_nan() || r.0.is_zero() || r.0.is_infinite() {
                assert_eq!(r.1, Equal);
            } else {
                assert!(r.0.get_prec().unwrap() <= prec);
            }
            // Multiplying x by 2^(1000k) multiplies the root by 2^(1000 sign(k)).
            if r.0.is_finite() && !r.0.is_zero() && k.unsigned_abs() <= 1000 {
                let s = 1000 * i64::exact_from(k.unsigned_abs());
                let (y, o) = root_s_rational_prec_round_all(&(&x << s), k, prec, rm);
                assert!(same(&y, &(&r.0 << if k > 0 { 1000 } else { -1000 })));
                assert_eq!(o, r.1);
            }
            // A dyadic x gives the root of the equal `BigExponentFloat`.
            if let Ok(x_bef) = BigExponentFloat::try_from(&x) {
                assert!(same_pair(&x_bef.root_s_prec_round(k, prec, rm), &r));
            }
            // `Float`'s function agrees wherever its result neither overflows nor underflows (at 0,
            // `Float` has signed zeros and infinities).
            if x != 0u32 {
                let (f, fo) = Float::root_s_rational_prec_round_ref(&x, k, prec, rm);
                if float_result_comparable(&f, fo) {
                    assert!(same(&r.0, &float_to_big_exponent_float(&f)));
                    assert_eq!(r.1, fo);
                }
            }
        },
    );
}
