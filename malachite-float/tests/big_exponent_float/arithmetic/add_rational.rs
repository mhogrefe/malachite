// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_float::big_exponent_float::ComparableBigExponentFloatRef;
use malachite_float::big_exponent_float::arithmetic::add_rational::add_rational_prec_round_generic;
use malachite_float::test_util::big_exponent_float::{
    big_exponent_float_exponent_is_small, float_to_big_exponent_float, parse_big_exponent_float_hex,
};
use malachite_float::test_util::generators::{
    big_exponent_float_rational_unsigned_rounding_mode_quadruple_gen_var_2,
    big_exponent_float_rational_unsigned_rounding_mode_quadruple_gen_var_2_float,
};
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

fn add_rational_prec_round_all(
    x: &BigExponentFloat,
    y: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = x.clone().add_rational_prec_round(y.clone(), prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(
        &x.clone().add_rational_prec_round_val_ref(y, prec, rm),
        &r
    ));
    assert!(same_pair(
        &x.add_rational_prec_round_ref_val(y.clone(), prec, rm),
        &r
    ));
    assert!(same_pair(
        &x.add_rational_prec_round_ref_ref(y, prec, rm),
        &r
    ));
    let mut z = x.clone();
    let o = z.add_rational_prec_round_assign(y.clone(), prec, rm);
    assert!(same_pair(&(z, o), &r));
    let mut z = x.clone();
    let o = z.add_rational_prec_round_assign_ref(y, prec, rm);
    assert!(same_pair(&(z, o), &r));
    if rm == Nearest {
        assert!(same_pair(&x.clone().add_rational_prec(y.clone(), prec), &r));
        assert!(same_pair(&x.clone().add_rational_prec_val_ref(y, prec), &r));
        assert!(same_pair(&x.add_rational_prec_ref_val(y.clone(), prec), &r));
        assert!(same_pair(&x.add_rational_prec_ref_ref(y, prec), &r));
        let mut z = x.clone();
        let o = z.add_rational_prec_assign(y.clone(), prec);
        assert!(same_pair(&(z, o), &r));
        let mut z = x.clone();
        let o = z.add_rational_prec_assign_ref(y, prec);
        assert!(same_pair(&(z, o), &r));
    }
    r
}

#[test]
fn test_add_rational_prec_round() {
    let check = |x: &str,
                 x_hex: &str,
                 y: &Rational,
                 prec: u64,
                 rm: RoundingMode,
                 out: &str,
                 out_hex: &str,
                 o| {
        assert_eq!(parse_big_exponent_float_hex(x_hex).to_string(), x);
        let x = &parse_big_exponent_float_hex(x_hex);
        assert_eq!(parse_big_exponent_float_hex(out_hex).to_string(), out);
        let out = &parse_big_exponent_float_hex(out_hex);
        let r = add_rational_prec_round_all(x, y, prec, rm);
        assert!(
            same(&r.0, out),
            "{x:?} {y} {prec} {rm}: {:?} != {out:?}",
            r.0
        );
        assert_eq!(r.1, o);
    };
    let third = Rational::from_unsigneds(1u8, 3);
    let neg_third = -&third;
    let q_zero = Rational::ZERO;
    // 1/3 rounded to 10 bits.
    check("NaN", "NaN", &third, 10, Floor, "NaN", "NaN", Equal);
    check(
        "Infinity", "Infinity", &third, 10, Floor, "Infinity", "Infinity", Equal,
    );
    check(
        "-Infinity",
        "-Infinity",
        &third,
        10,
        Floor,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    check("0.0", "0x0.0", &q_zero, 10, Floor, "0.0", "0x0.0", Equal);
    check("0.0", "0x0.0", &third, 10, Floor, "0.3330", "0x0.554", Less);
    check("1.0", "0x1.0", &q_zero, 10, Floor, "1.0", "0x1.0", Equal);
    check(
        "1.0",
        "0x1.0",
        &Rational::ONE,
        1,
        Exact,
        "2.0",
        "0x2.0",
        Equal,
    );
    check(
        "1.0",
        "0x1.0",
        &-Rational::ONE,
        1,
        Exact,
        "0.0",
        "0x0.0",
        Equal,
    );
    // 1/3 is far below 2^N: it decides only the rounding direction.
    check(
        "2.4e30102999566398119521",
        "0x1.0E+25000000000000000000",
        &third,
        10,
        Floor,
        "2.4e30102999566398119521",
        "0x1.0E+25000000000000000000",
        Less,
    );
    check(
        "2.4e30102999566398119521",
        "0x1.0E+25000000000000000000",
        &third,
        10,
        Nearest,
        "2.4e30102999566398119521",
        "0x1.0E+25000000000000000000",
        Less,
    );
    check(
        "2.4e30102999566398119521",
        "0x1.0E+25000000000000000000",
        &third,
        10,
        Ceiling,
        "2.3699e30102999566398119521",
        "0x1.008E+25000000000000000000",
        Greater,
    );
    check(
        "2.4e30102999566398119521",
        "0x1.0E+25000000000000000000",
        &neg_third,
        10,
        Floor,
        "2.3630e30102999566398119521",
        "0xf.fcE+24999999999999999999",
        Less,
    );
    // 2^-N is far below 1/3: it decides only the rounding direction.
    check(
        "4.2e-30102999566398119522",
        "0x1.0E-25000000000000000000",
        &third,
        10,
        Floor,
        "0.3330",
        "0x0.554",
        Less,
    );
    check(
        "4.2e-30102999566398119522",
        "0x1.0E-25000000000000000000",
        &third,
        10,
        Ceiling,
        "0.33350",
        "0x0.556",
        Greater,
    );
    check(
        "-4.2e-30102999566398119522",
        "-0x1.0E-25000000000000000000",
        &third,
        10,
        Floor,
        "0.3330",
        "0x0.554",
        Less,
    );
    check(
        "-4.2e-30102999566398119522",
        "-0x1.0E-25000000000000000000",
        &third,
        10,
        Ceiling,
        "0.33350",
        "0x0.556",
        Greater,
    );
}

#[test]
#[should_panic]
fn add_rational_prec_round_fail_1() {
    BigExponentFloat::ONE.add_rational_prec_round(Rational::ONE, 0, Floor);
}

#[test]
#[should_panic]
fn add_rational_prec_round_fail_2() {
    BigExponentFloat::ONE.add_rational_prec_round(Rational::from_unsigneds(1u8, 3), 10, Exact);
}

#[test]
fn add_rational_prec_round_properties() {
    big_exponent_float_rational_unsigned_rounding_mode_quadruple_gen_var_2().test_properties(
        |(x, y, prec, rm)| {
            let r = add_rational_prec_round_all(&x, &y, prec, rm);
            // The kernels agree with the kernel-free implementation, at any exponents.
            assert!(same_pair(
                &add_rational_prec_round_generic(&x, &y, prec, rm),
                &r
            ));
            if r.0.is_nan() || r.0.is_zero() || r.0.is_infinite() {
                assert_eq!(r.1, Equal);
            } else {
                assert!(r.0.get_prec().unwrap() <= prec);
            }
            // x + y = x - (-y).
            assert!(same_pair(
                &x.sub_rational_prec_round_ref_val(-&y, prec, rm),
                &r
            ));
            // (-x) + (-y) = -(x + y), with the rounding mode and the ordering reversed.
            let (s, o) = (-&x).add_rational_prec_round(-&y, prec, -rm);
            assert!(same(&-s, &r.0));
            assert_eq!(o.reverse(), r.1);
            // A dyadic y is added like a `BigExponentFloat`.
            if let Ok(y) = BigExponentFloat::try_from(&y) {
                assert!(same_pair(&x.add_prec_round_ref_ref(&y, prec, rm), &r));
            }
            if big_exponent_float_exponent_is_small(&x) && x.is_finite() {
                let q = Rational::exact_from(&x) + &y;
                assert_eq!(Rational::exact_from(&r.0).cmp(&q), r.1);
            }
        },
    );
}

#[test]
fn add_rational_prec_round_float_agreement_properties() {
    big_exponent_float_rational_unsigned_rounding_mode_quadruple_gen_var_2_float().test_properties(
        |((fx, y, prec, rm), (x, _, _, _))| {
            let (f, fo) = fx.add_rational_prec_round_ref_ref(&y, prec, rm);
            if float_result_comparable(&f, fo) {
                let r = add_rational_prec_round_all(&x, &y, prec, rm);
                assert!(same(&r.0, &float_to_big_exponent_float(&f)));
                assert_eq!(r.1, fo);
            }
        },
    );
}
