// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.
use malachite_base::num::arithmetic::traits::{Square, SquareAssign};
use malachite_base::num::basic::traits::One;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_float::big_exponent_float::ComparableBigExponentFloatRef;
use malachite_float::big_exponent_float::arithmetic::square::square_prec_round_generic;
use malachite_float::test_util::big_exponent_float::{
    big_exponent_float_exponent_is_small, big_exponent_float_negate, float_to_big_exponent_float,
    parse_big_exponent_float_hex,
};
use malachite_float::test_util::generators::{
    big_exponent_float_gen, big_exponent_float_unsigned_rounding_mode_triple_gen_var_2,
    float_unsigned_rounding_mode_triple_gen_var_2,
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

fn square_prec_round_all(
    x: &BigExponentFloat,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = x.clone().square_prec_round(prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(&x.square_prec_round_ref(prec, rm), &r));
    let mut z = x.clone();
    let o = z.square_prec_round_assign(prec, rm);
    assert!(same_pair(&(z, o), &r));
    if rm == Nearest {
        assert!(same_pair(&x.clone().square_prec(prec), &r));
        assert!(same_pair(&x.square_prec_ref(prec), &r));
        let mut z = x.clone();
        let o = z.square_prec_assign(prec);
        assert!(same_pair(&(z, o), &r));
    }
    r
}

fn square_all(x: &BigExponentFloat) -> BigExponentFloat {
    let r = x.clone().square();
    assert!(r.is_valid());
    assert!(same(&x.square(), &r));
    let mut z = x.clone();
    z.square_assign();
    assert!(same(&z, &r));
    r
}

#[test]
fn test_square() {
    let pi = parse_big_exponent_float_hex("0x3.243f6a8885a30#53");
    let check = |x: &str, x_hex: &str, prec: u64, rm: RoundingMode, out: &str, out_hex: &str, o| {
        assert_eq!(parse_big_exponent_float_hex(x_hex).to_string(), x);
        let x = &parse_big_exponent_float_hex(x_hex);
        assert_eq!(parse_big_exponent_float_hex(out_hex).to_string(), out);
        let out = &parse_big_exponent_float_hex(out_hex);
        let r = square_prec_round_all(x, prec, rm);
        assert!(same(&r.0, out), "{x:?} {prec} {rm}: {:?} != {out:?}", r.0);
        assert_eq!(r.1, o);
    };
    check("NaN", "NaN", 10, Floor, "NaN", "NaN", Equal);
    check(
        "-Infinity",
        "-Infinity",
        10,
        Floor,
        "Infinity",
        "Infinity",
        Equal,
    );
    check("0.0", "0x0.0", 10, Floor, "0.0", "0x0.0", Equal);
    check("-1.0", "-0x1.0", 1, Exact, "1.0", "0x1.0", Equal);
    check(
        "2.4e30102999566398119521",
        "0x1.0E+25000000000000000000",
        1,
        Exact,
        "5.6e60205999132796239042",
        "0x1.0E+50000000000000000000",
        Equal,
    );
    check(
        "3.1415926535897931",
        "0x3.243f6a8885a3",
        5,
        Floor,
        "9.50",
        "0x9.8",
        Less,
    );
    check(
        "3.1415926535897931",
        "0x3.243f6a8885a3",
        20,
        Nearest,
        "9.869598",
        "0x9.de9e",
        Less,
    );
    let s = square_all(&pi);
    assert_eq!(Rational::exact_from(&s), Rational::exact_from(&pi).square());
    assert!(same(
        &square_all(&BigExponentFloat::from(-1.5)),
        &BigExponentFloat::from(2.25)
    ));
}

#[test]
#[should_panic]
fn square_prec_round_fail_1() {
    BigExponentFloat::ONE.square_prec_round(0, Floor);
}

#[test]
#[should_panic]
fn square_prec_round_fail_2() {
    BigExponentFloat::from(1.5).square_prec_round(1, Exact);
}

#[test]
fn square_prec_round_properties() {
    big_exponent_float_unsigned_rounding_mode_triple_gen_var_2().test_properties(
        |(x, prec, rm)| {
            let r = square_prec_round_all(&x, prec, rm);
            assert!(same_pair(&square_prec_round_generic(&x, prec, rm), &r));
            assert!(same_pair(&x.mul_prec_round_ref_ref(&x, prec, rm), &r));
            assert!(same_pair(
                &big_exponent_float_negate(x.clone()).square_prec_round(prec, rm),
                &r
            ));
            if !r.0.is_nan() {
                assert!(r.0.is_sign_positive());
            }
            if big_exponent_float_exponent_is_small(&x) {
                let exact = square_all(&x);
                assert!(same_pair(&exact.round_to_prec_round_ref(prec, rm), &r));
                if exact.is_finite() {
                    let q = Rational::exact_from(&x).square();
                    assert_eq!(Rational::exact_from(&exact), q);
                    assert_eq!(Rational::exact_from(&r.0).cmp(&q), r.1);
                }
            }
        },
    );
}

#[test]
fn square_properties() {
    big_exponent_float_gen().test_properties(|x| {
        let s = square_all(&x);
        assert!(same(&s, &(&x * &x)));
    });
}

#[test]
fn square_prec_round_float_agreement_properties() {
    float_unsigned_rounding_mode_triple_gen_var_2().test_properties(|(x, prec, rm)| {
        let (f, fo) = x.square_prec_round_ref(prec, rm);
        if float_result_comparable(&f, fo) {
            let r = square_prec_round_all(&float_to_big_exponent_float(&x), prec, rm);
            assert!(same(&r.0, &float_to_big_exponent_float(&f)));
            assert_eq!(r.1, fo);
        }
    });
}
