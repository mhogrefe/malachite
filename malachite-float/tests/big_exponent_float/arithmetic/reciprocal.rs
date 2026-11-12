// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::Reciprocal;
use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity, One, Zero};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_float::big_exponent_float::ComparableBigExponentFloatRef;
use malachite_float::big_exponent_float::arithmetic::reciprocal::reciprocal_prec_round_generic;
use malachite_float::test_util::big_exponent_float::{
    big_exponent_float_exponent_is_small, big_exponent_float_power_of_2,
    float_to_big_exponent_float, parse_big_exponent_float_hex,
};
use malachite_float::test_util::generators::{
    big_exponent_float_unsigned_rounding_mode_triple_gen_var_3,
    float_unsigned_rounding_mode_triple_gen_var_3,
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

fn float_result_comparable(x: &Float, o: Ordering) -> bool {
    x.get_exponent().map_or_else(
        || !x.is_infinite() && (!x.is_zero() || o == Equal),
        |e| e > Float::MIN_EXPONENT + 1 && e < Float::MAX_EXPONENT - 1,
    )
}

fn reciprocal_prec_round_all(
    x: &BigExponentFloat,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = x.clone().reciprocal_prec_round(prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(&x.reciprocal_prec_round_ref(prec, rm), &r));
    let mut z = x.clone();
    let o = z.reciprocal_prec_round_assign(prec, rm);
    assert!(same_pair(&(z, o), &r));
    if rm == Nearest {
        assert!(same_pair(&x.clone().reciprocal_prec(prec), &r));
        assert!(same_pair(&x.reciprocal_prec_ref(prec), &r));
        let mut z = x.clone();
        let o = z.reciprocal_prec_assign(prec);
        assert!(same_pair(&(z, o), &r));
    }
    r
}

#[test]
fn test_reciprocal_prec_round() {
    let big = Integer::from_str("100000000000000000000").unwrap();
    let pi = parse_big_exponent_float_hex("0x3.243f6a8885a30#53");
    let check = |x: &BigExponentFloat, prec: u64, rm: RoundingMode, out: &BigExponentFloat, o| {
        let r = reciprocal_prec_round_all(x, prec, rm);
        assert!(same(&r.0, out), "{x:?} {prec} {rm}: {:?} != {out:?}", r.0);
        assert_eq!(r.1, o);
    };
    let nan = BigExponentFloat::NAN;
    let zero = BigExponentFloat::ZERO;
    check(&nan, 10, Floor, &nan, Equal);
    check(&zero, 10, Floor, &nan, Equal);
    check(&BigExponentFloat::INFINITY, 10, Floor, &zero, Equal);
    check(
        &BigExponentFloat::NEGATIVE_INFINITY,
        10,
        Floor,
        &zero,
        Equal,
    );
    check(
        &BigExponentFloat::ONE,
        1,
        Exact,
        &BigExponentFloat::ONE,
        Equal,
    );
    check(
        &BigExponentFloat::from(-4.0),
        1,
        Exact,
        &BigExponentFloat::from(-0.25),
        Equal,
    );
    check(
        &big_exponent_float_power_of_2(big.clone()),
        1,
        Exact,
        &big_exponent_float_power_of_2(-big.clone()),
        Equal,
    );
    check(
        &big_exponent_float_power_of_2(-big.clone()),
        1,
        Exact,
        &big_exponent_float_power_of_2(big),
        Equal,
    );
    check(
        &pi,
        5,
        Ceiling,
        &parse_big_exponent_float_hex("0x0.54#5"),
        Greater,
    );
    check(
        &pi,
        20,
        Nearest,
        &parse_big_exponent_float_hex("0x0.517cc0#20"),
        Less,
    );
}

#[test]
#[should_panic]
fn reciprocal_prec_round_fail_1() {
    BigExponentFloat::ONE.reciprocal_prec_round(0, Floor);
}

#[test]
#[should_panic]
fn reciprocal_prec_round_fail_2() {
    BigExponentFloat::from(3.0).reciprocal_prec_round(10, Exact);
}

#[test]
fn reciprocal_prec_round_properties() {
    big_exponent_float_unsigned_rounding_mode_triple_gen_var_3().test_properties(
        |(x, prec, rm)| {
            let r = reciprocal_prec_round_all(&x, prec, rm);
            // The kernels agree with the kernel-free implementation, at any exponents.
            assert!(same_pair(&reciprocal_prec_round_generic(&x, prec, rm), &r));
            assert!(same_pair(
                &BigExponentFloat::ONE.div_prec_round_val_ref(&x, prec, rm),
                &r
            ));
            if r.0.is_nan() || r.0.is_zero() {
                assert_eq!(r.1, Equal);
            } else {
                assert!(r.0.get_prec().unwrap() <= prec);
            }
            // 1/(-x) = -(1/x), with the rounding mode and the ordering reversed.
            let (q, o) = (-&x).reciprocal_prec_round(prec, -rm);
            assert!(same(&-q, &r.0));
            assert_eq!(o.reverse(), r.1);
            if big_exponent_float_exponent_is_small(&x) && x.is_finite() && !x.is_zero() {
                let q = Rational::exact_from(&x).reciprocal();
                assert_eq!(Rational::exact_from(&r.0).cmp(&q), r.1);
            }
        },
    );
}

#[test]
fn reciprocal_prec_round_float_agreement_properties() {
    float_unsigned_rounding_mode_triple_gen_var_3().test_properties(|(x, prec, rm)| {
        let (f, fo) = x.reciprocal_prec_round_ref(prec, rm);
        if float_result_comparable(&f, fo) {
            let r = reciprocal_prec_round_all(&float_to_big_exponent_float(&x), prec, rm);
            assert!(same(&r.0, &float_to_big_exponent_float(&f)));
            assert_eq!(r.1, fo);
        }
    });
}
