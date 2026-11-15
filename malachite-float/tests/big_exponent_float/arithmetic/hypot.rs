// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::Square;
use malachite_base::num::basic::traits::One;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_float::big_exponent_float::ComparableBigExponentFloatRef;
use malachite_float::big_exponent_float::arithmetic::hypot::hypot_prec_round_generic;
use malachite_float::test_util::big_exponent_float::{
    big_exponent_float_exponent_is_small, float_to_big_exponent_float, parse_big_exponent_float_hex,
};
use malachite_float::test_util::generators::{
    big_exponent_float_big_exponent_float_unsigned_rounding_mode_quadruple_gen_var_6,
    float_float_unsigned_rounding_mode_quadruple_gen_var_24,
};
use malachite_float::{BigExponentFloat, Float};
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

// Whether `x` is finite and nonzero.
const fn finite_nonzero(x: &BigExponentFloat) -> bool {
    x.is_finite() && !x.is_zero()
}

fn hypot_prec_round_all(
    x: &BigExponentFloat,
    y: &BigExponentFloat,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = x.clone().hypot_prec_round(y.clone(), prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(
        &x.clone().hypot_prec_round_val_ref(y, prec, rm),
        &r
    ));
    assert!(same_pair(
        &x.hypot_prec_round_ref_val(y.clone(), prec, rm),
        &r
    ));
    assert!(same_pair(&x.hypot_prec_round_ref_ref(y, prec, rm), &r));
    let mut z = x.clone();
    let o = z.hypot_prec_round_assign(y.clone(), prec, rm);
    assert!(same_pair(&(z, o), &r));
    let mut z = x.clone();
    let o = z.hypot_prec_round_assign_ref(y, prec, rm);
    assert!(same_pair(&(z, o), &r));
    if rm == Nearest {
        assert!(same_pair(&x.clone().hypot_prec(y.clone(), prec), &r));
        assert!(same_pair(&x.clone().hypot_prec_val_ref(y, prec), &r));
        assert!(same_pair(&x.hypot_prec_ref_val(y.clone(), prec), &r));
        assert!(same_pair(&x.hypot_prec_ref_ref(y, prec), &r));
        let mut z = x.clone();
        let o = z.hypot_prec_assign(y.clone(), prec);
        assert!(same_pair(&(z, o), &r));
        let mut z = x.clone();
        let o = z.hypot_prec_assign_ref(y, prec);
        assert!(same_pair(&(z, o), &r));
    }
    r
}

fn check(
    x: &str,
    x_hex: &str,
    y: &str,
    y_hex: &str,
    prec: u64,
    rm: RoundingMode,
    out: &str,
    out_hex: &str,
    o: Ordering,
) {
    assert_eq!(parse_big_exponent_float_hex(x_hex).to_string(), x);
    let x = &parse_big_exponent_float_hex(x_hex);
    assert_eq!(parse_big_exponent_float_hex(y_hex).to_string(), y);
    let y = &parse_big_exponent_float_hex(y_hex);
    assert_eq!(parse_big_exponent_float_hex(out_hex).to_string(), out);
    let out = &parse_big_exponent_float_hex(out_hex);
    let r = hypot_prec_round_all(x, y, prec, rm);
    assert!(
        same(&r.0, out),
        "{x:?} {y:?} {prec} {rm}: {:?} != {out:?}",
        r.0
    );
    assert_eq!(r.1, o);
    assert!(same_pair(&hypot_prec_round_generic(x, y, prec, rm), &r));
}

fn check_pi(
    x: &str,
    x_hex: &str,
    prec: u64,
    rm: RoundingMode,
    out: &str,
    out_hex: &str,
    o: Ordering,
) {
    // y = e, as an f64
    check(
        x,
        x_hex,
        "2.7182818284590451",
        "0x2.b7e151628aed2",
        prec,
        rm,
        out,
        out_hex,
        o,
    );
}

#[test]
fn test_hypot_prec_round() {
    check(
        "NaN",
        "NaN",
        "-Infinity",
        "-Infinity",
        10,
        Floor,
        "Infinity",
        "Infinity",
        Equal,
    );
    check(
        "-Infinity",
        "-Infinity",
        "NaN",
        "NaN",
        10,
        Floor,
        "Infinity",
        "Infinity",
        Equal,
    );
    check("NaN", "NaN", "1.0", "0x1.0", 10, Floor, "NaN", "NaN", Equal);
    check("1.0", "0x1.0", "NaN", "NaN", 10, Floor, "NaN", "NaN", Equal);
    check(
        "0.0", "0x0.0", "0.0", "0x0.0", 10, Floor, "0.0", "0x0.0", Equal,
    );
    check(
        "0.0", "0x0.0", "-3.0", "-0x3.0", 10, Floor, "3.0", "0x3.0", Equal,
    );
    check(
        "-3.0", "-0x3.0", "0.0", "0x0.0", 1, Floor, "2.0", "0x2.0", Less,
    );
    check(
        "3.0", "0x3.0", "-4.0", "-0x4.0", 3, Exact, "5.0", "0x5.0", Equal,
    );
    // Exponents beyond the range of a `Float`: hypot(3 * 2^N, 4 * 2^N) = 5 * 2^N.
    check(
        "7.1e30102999566398119521",
        "0x3.0E+25000000000000000000",
        "9.5e30102999566398119521",
        "0x4.0E+25000000000000000000",
        3,
        Exact,
        "1.2e30102999566398119522",
        "0x5.0E+25000000000000000000",
        Equal,
    );
    // y far below x: the hypotenuse is just above |x|.
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
        Nearest,
        "1.0",
        "0x1.0",
        Less,
    );
    check(
        "4.2e-30102999566398119522",
        "0x1.0E-25000000000000000000",
        "1.0",
        "0x1.0",
        10,
        Ceiling,
        "1.0020",
        "0x1.008",
        Greater,
    );
    check(
        "2.4e30102999566398119521",
        "0x1.0E+25000000000000000000",
        "-1.0",
        "-0x1.0",
        1,
        Up,
        "4.7e30102999566398119521",
        "0x2.0E+25000000000000000000",
        Greater,
    );
    // 1 + 2^-9 has 10 bits; at 9 bits it is a midpoint, but the hypotenuse is just above it.
    check(
        "1.0020",
        "0x1.008",
        "4.2e-30102999566398119522",
        "0x1.0E-25000000000000000000",
        9,
        Nearest,
        "1.004",
        "0x1.01",
        Greater,
    );
    check(
        "1.0020",
        "0x1.008",
        "4.2e-30102999566398119522",
        "0x1.0E-25000000000000000000",
        10,
        Nearest,
        "1.0020",
        "0x1.008",
        Less,
    );
    check_pi(
        "3.1415926535897931",
        "0x3.243f6a8885a3",
        5,
        Floor,
        "4.0",
        "0x4.0",
        Less,
    );
    check_pi(
        "3.1415926535897931",
        "0x3.243f6a8885a3",
        5,
        Ceiling,
        "4.25",
        "0x4.4",
        Greater,
    );
    check_pi(
        "3.1415926535897931",
        "0x3.243f6a8885a3",
        20,
        Nearest,
        "4.154358",
        "0x4.2784",
        Greater,
    );
}

#[test]
#[should_panic]
fn hypot_prec_round_fail_1() {
    BigExponentFloat::ONE.hypot_prec_round(BigExponentFloat::ONE, 0, Floor);
}

#[test]
#[should_panic]
fn hypot_prec_round_fail_2() {
    BigExponentFloat::ONE.hypot_prec_round(BigExponentFloat::ONE, 10, Exact);
}

#[test]
fn hypot_prec_round_properties() {
    big_exponent_float_big_exponent_float_unsigned_rounding_mode_quadruple_gen_var_6()
        .test_properties(|(x, y, prec, rm)| {
            let r = hypot_prec_round_all(&x, &y, prec, rm);
            // The kernels agree with the kernel-free implementation, at any exponents.
            assert!(same_pair(&hypot_prec_round_generic(&x, &y, prec, rm), &r));
            if r.0.is_nan() || r.0.is_zero() || r.0.is_infinite() {
                assert_eq!(r.1, Equal);
            } else {
                assert!(r.0.is_sign_positive());
                assert!(r.0.get_prec().unwrap() <= prec);
            }
            assert!(same_pair(&hypot_prec_round_all(&y, &x, prec, rm), &r));
            assert!(same_pair(&hypot_prec_round_all(&-&x, &y, prec, rm), &r));
            // Multiplying both arguments by 2 doubles the hypotenuse.
            let (s, o) = (&x << 1u32).hypot_prec_round(&y << 1u32, prec, rm);
            assert!(same(&s, &(&r.0 << 1u32)));
            assert_eq!(o, r.1);
            if finite_nonzero(&x)
                && finite_nonzero(&y)
                && big_exponent_float_exponent_is_small(&x)
                && big_exponent_float_exponent_is_small(&y)
            {
                // Comparing squares gives the ordering.
                assert_eq!(
                    (&r.0)
                        .square()
                        .partial_cmp(&((&x).square() + (&y).square())),
                    Some(r.1)
                );
            }
        });
}

#[test]
fn hypot_prec_round_float_agreement_properties() {
    float_float_unsigned_rounding_mode_quadruple_gen_var_24().test_properties(
        |(x, y, prec, rm)| {
            let (f, fo) = x.hypot_prec_round_ref_ref(&y, prec, rm);
            if float_result_comparable(&f, fo) {
                let r = hypot_prec_round_all(
                    &float_to_big_exponent_float(&x),
                    &float_to_big_exponent_float(&y),
                    prec,
                    rm,
                );
                assert!(same(&r.0, &float_to_big_exponent_float(&f)));
                assert_eq!(r.1, fo);
            }
        },
    );
}
